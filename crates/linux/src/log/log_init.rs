use anyhow::{bail, Result};
use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use std::fs::create_dir_all;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

type SharedSender = Arc<Mutex<Option<crossbeam::channel::Sender<Vec<u8>>>>>;

static LOG_GUARD: std::sync::OnceLock<Arc<NonBlockingGuard>> = std::sync::OnceLock::new();

/// Configuration for the logger.
///
/// Supports a fluent builder API for easy setup.  Call [`LogConfig::new`] to
/// create an instance, chain the desired `with_*` methods, and then call
/// [`LogConfig::init`] to activate the global subscriber.
///
/// # Examples
///
/// ```no_run
/// use alumy_linux::LogConfig;
///
/// LogConfig::new("my-app", "info")
///     .with_file("logs/app.log", "10M", 5)
///     .with_target(true)
///     .init()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct LogConfig {
    /// Application name used to identify this logger instance.
    pub name: Option<String>,
    /// Path to the log file.  When `None`, output goes to stdout.
    pub file: Option<String>,
    /// Minimum log level (e.g. `"info"`, `"debug"`, `"warn"`).
    pub level: Option<String>,
    /// Maximum size of a single log file before rotation (e.g. `"10M"`).
    /// Required when [`file`](Self::file) is set.
    pub max_size: Option<String>,
    /// Maximum number of rotated log files to retain.
    /// Required when [`file`](Self::file) is set.
    pub max_files: Option<u32>,
    /// Custom [`EnvFilter`](tracing_subscriber::EnvFilter) directive string
    /// (e.g. `"info,my_crate=debug"`).  Overrides [`level`](Self::level) when set.
    pub filter: Option<String>,
    /// Whether to emit ANSI colour codes.  Defaults to `true` for stdout,
    /// `false` for file output.
    pub ansi: Option<bool>,
    /// Whether to include the tracing target (module path) in each log line.
    pub display_target: Option<bool>,
    /// Whether to include the log level in each log line.  Defaults to `true`.
    pub display_level: Option<bool>,
    /// Whether to include a timestamp in each log line.  Defaults to `true`.
    pub display_time: Option<bool>,
    /// Whether to include the thread name in each log line.
    pub display_thread_name: Option<bool>,
    /// Whether to include the thread ID in each log line.
    pub display_thread_id: Option<bool>,
    /// Timestamp format.  Use `"uptime"` for elapsed seconds since boot, or
    /// `"iso"` (default) for a local-time ISO 8601 string.
    pub time_format: Option<String>,
}

impl LogConfig {
    /// Creates a new log configuration with the required `name` and `level`.
    pub fn new(name: impl Into<String>, level: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            level: Some(level.into()),
            ..Default::default()
        }
    }

    /// Sets the log file path and rolling policy.
    pub fn with_file(
        mut self,
        path: impl Into<String>,
        max_size: impl Into<String>,
        max_files: u32,
    ) -> Self {
        self.file = Some(path.into());
        self.max_size = Some(max_size.into());
        self.max_files = Some(max_files);
        self
    }

    /// Sets a custom tracing filter directive (e.g. `"info,my_crate=debug"`).
    ///
    /// When set, this overrides the [`level`](Self::level) field.
    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Enables or disables ANSI colour codes in log output.
    pub fn with_ansi(mut self, enable: bool) -> Self {
        self.ansi = Some(enable);
        self
    }

    /// Enables or disables the tracing target (module path) in log lines.
    pub fn with_target(mut self, enable: bool) -> Self {
        self.display_target = Some(enable);
        self
    }

    /// Enables or disables the log level label in log lines.
    pub fn with_level_display(mut self, enable: bool) -> Self {
        self.display_level = Some(enable);
        self
    }

    /// Enables or disables the timestamp in log lines.
    pub fn with_time(mut self, enable: bool) -> Self {
        self.display_time = Some(enable);
        self
    }

    /// Sets the timestamp format.
    ///
    /// Use `"uptime"` for elapsed seconds since boot, or `"iso"` (default)
    /// for a local-time ISO 8601 string.
    pub fn with_time_format(mut self, format: impl Into<String>) -> Self {
        self.time_format = Some(format.into());
        self
    }

    /// Enables or disables thread names in log lines.
    pub fn with_thread_name(mut self, enable: bool) -> Self {
        self.display_thread_name = Some(enable);
        self
    }

    /// Enables or disables thread IDs in log lines.
    pub fn with_thread_id(mut self, enable: bool) -> Self {
        self.display_thread_id = Some(enable);
        self
    }

    /// Initialises the global logger with this configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields (`name`, `level`) are missing, if
    /// `file` is set but `max_size` or `max_files` are not, or if the log
    /// directory cannot be created or the rolling appender cannot be opened.
    pub fn init(&self) -> Result<()> {
        logger_init(self)
    }

    // Helper accessors with defaults for internal use.
    fn display_target(&self) -> bool {
        self.display_target.unwrap_or(false)
    }
    fn display_level(&self) -> bool {
        self.display_level.unwrap_or(true)
    }
    fn display_time(&self) -> bool {
        self.display_time.unwrap_or(true)
    }
    fn display_thread_name(&self) -> bool {
        self.display_thread_name.unwrap_or(false)
    }
    fn display_thread_id(&self) -> bool {
        self.display_thread_id.unwrap_or(false)
    }
    fn time_format(&self) -> &str {
        self.time_format.as_deref().unwrap_or("iso")
    }
}

struct UptimeTime;

impl tracing_subscriber::fmt::time::FormatTime for UptimeTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let duration = crate::sys::uptime::uptime_duration();
        write!(
            w,
            "[{:>6}.{:03}]",
            duration.as_secs(),
            duration.subsec_millis()
        )
    }
}

pub struct NonBlockingWriter {
    sender: SharedSender,
}

impl NonBlockingWriter {
    pub fn new<W: Write + Send + 'static>(mut writer: W) -> (Self, NonBlockingGuard) {
        let (sender, receiver) = crossbeam::channel::unbounded::<Vec<u8>>();
        let shared_sender: SharedSender = Arc::new(Mutex::new(Some(sender)));

        let handle = thread::Builder::new()
            .name("tracing-writer".to_string())
            .stack_size(2 * 1024 * 1024)
            .spawn(move || {
                for msg in receiver {
                    let _ = writer.write_all(&msg);
                    let _ = writer.flush();
                }
            })
            .expect("Failed to spawn logging thread");

        let guard = NonBlockingGuard {
            sender: shared_sender.clone(),
            handle: Some(handle),
        };

        (
            NonBlockingWriter {
                sender: shared_sender,
            },
            guard,
        )
    }
}

impl Clone for NonBlockingWriter {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

pub struct NonBlockingWriterHandle {
    sender: SharedSender,
    buffer: Vec<u8>,
}

impl Write for NonBlockingWriterHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if !self.buffer.is_empty() {
            if let Ok(guard) = self.sender.lock() {
                if let Some(ref sender) = *guard {
                    let _ = sender.send(std::mem::take(&mut self.buffer));
                }
            }
        }
        Ok(())
    }
}

impl Drop for NonBlockingWriterHandle {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl<'a> MakeWriter<'a> for NonBlockingWriter {
    type Writer = NonBlockingWriterHandle;

    fn make_writer(&'a self) -> Self::Writer {
        NonBlockingWriterHandle {
            sender: self.sender.clone(),
            buffer: Vec::with_capacity(256),
        }
    }
}

pub struct NonBlockingGuard {
    sender: SharedSender,
    handle: Option<JoinHandle<()>>,
}

impl Drop for NonBlockingGuard {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.sender.lock() {
            *guard = None;
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn log_config_check(log_config: &LogConfig) -> Result<()> {
    if log_config.name.is_none() {
        bail!("Log name is required");
    }

    if log_config.level.is_none() {
        bail!("Log level is required");
    }

    if log_config.file.is_some() {
        if log_config.max_size.is_none() {
            bail!("Log max size is required");
        }

        if log_config.max_files.is_none() {
            bail!("Log max files is required");
        }
    }

    Ok(())
}

fn log_dir_create(log_config: &LogConfig) -> Result<()> {
    if let Some(file) = log_config.file.as_deref() {
        let path = Path::new(file);

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        }
    }
    Ok(())
}

macro_rules! subscriber_init {
    ($env_filter:expr, $layer:expr, $cfg:expr) => {
        let registry = tracing_subscriber::registry().with($env_filter);
        if !$cfg.display_time() {
            let _ = registry.with($layer.without_time()).try_init();
        } else if $cfg.time_format() == "uptime" {
            let _ = registry.with($layer.with_timer(UptimeTime)).try_init();
        } else {
            let _ = registry
                .with($layer.with_timer(fmt::time::LocalTime::new(
                    time::macros::format_description!(
                        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
                    ),
                )))
                .try_init();
        }
    };
}

/// Initialises the global logger.
///
/// This is an internal function used by [`LogConfig::init`].
pub(crate) fn logger_init(log_config: &LogConfig) -> Result<()> {
    log_config_check(log_config).inspect_err(|e| eprintln!("Failed to check log config: {e}"))?;
    log_dir_create(log_config).inspect_err(|e| eprintln!("Failed to create log directory: {e}"))?;

    let env_filter = log_config
        .filter
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|f| EnvFilter::try_new(f).unwrap_or_else(|_| EnvFilter::new("info")))
        .unwrap_or_else(|| EnvFilter::new(log_config.level.as_deref().unwrap_or("info")));

    if let Some(file) = log_config.file.as_deref() {
        let file_path = Path::new(file);

        let dir = file_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        let basename = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| log_config.name.as_deref().unwrap_or("alumy"));

        let max_size = log_config
            .max_size
            .as_deref()
            .and_then(alumy_core::fs::filesize::parse_size)
            .unwrap_or(1024 * 1024);

        let rolling_appender = BasicRollingFileAppender::new(
            dir.join(format!("{basename}.log")),
            RollingConditionBasic::new().max_size(max_size),
            log_config.max_files.unwrap_or(5).max(2) as usize,
        )
        .map_err(|e| anyhow::anyhow!("Failed to create rolling file appender: {e}"))?;

        let (non_blocking, guard) = NonBlockingWriter::new(rolling_appender);
        LOG_GUARD.get_or_init(|| Arc::new(guard));

        let layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(log_config.ansi.unwrap_or(false))
            .with_target(log_config.display_target())
            .with_level(log_config.display_level())
            .with_thread_names(log_config.display_thread_name())
            .with_thread_ids(log_config.display_thread_id());

        subscriber_init!(env_filter, layer, log_config);
    } else {
        let layer = fmt::layer()
            .with_ansi(log_config.ansi.unwrap_or(true))
            .with_target(log_config.display_target())
            .with_level(log_config.display_level())
            .with_thread_names(log_config.display_thread_name())
            .with_thread_ids(log_config.display_thread_id());

        subscriber_init!(env_filter, layer, log_config);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_config_fluent_api() {
        let config = LogConfig::new("test", "info")
            .with_file("test.log", "10M", 5)
            .with_ansi(true)
            .with_target(true);

        assert_eq!(config.name.as_deref(), Some("test"));
        assert_eq!(config.level.as_deref(), Some("info"));
        assert_eq!(config.file.as_deref(), Some("test.log"));
        assert_eq!(config.ansi, Some(true));
        assert_eq!(config.display_target, Some(true));
    }

    #[test]
    fn test_log_config_check() {
        let config = LogConfig::new("test", "info");
        assert!(log_config_check(&config).is_ok());

        let err_config = LogConfig::default();
        assert!(log_config_check(&err_config).is_err());
    }

    #[test]
    fn test_logger_init_errors() {
        let config = LogConfig::default();
        assert!(logger_init(&config).is_err());
    }
}
