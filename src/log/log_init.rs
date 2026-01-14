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

#[derive(Debug, Clone, Default)]
pub struct LogConfig {
    pub name: Option<String>,
    pub file: Option<String>,
    pub level: Option<String>,
    pub max_size: Option<String>,
    pub max_files: Option<u32>,
    pub filter: Option<String>,
    pub ansi: Option<bool>,
    pub display_target: Option<bool>,
    pub display_level: Option<bool>,
    pub display_time: Option<bool>,
    pub display_thread_name: Option<bool>,
    pub display_thread_id: Option<bool>,
    pub time_format: Option<String>,
}

impl LogConfig {
    pub fn new(
        name: Option<String>,
        file: Option<String>,
        level: Option<String>,
        max_size: Option<String>,
        max_files: Option<u32>,
    ) -> Self {
        Self {
            name,
            file,
            level,
            max_size,
            max_files,
            ..Default::default()
        }
    }

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
        write!(w, "[{:>6}.{:03}]", duration.as_secs(), duration.subsec_millis())
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

        (NonBlockingWriter { sender: shared_sender }, guard)
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
    if let Some(parent) = log_config.file.as_deref().and_then(|f| Path::new(f).parent()) {
        create_dir_all(parent)?;
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
                .with($layer.with_timer(fmt::time::LocalTime::new(time::macros::format_description!(
                    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
                ))))
                .try_init();
        }
    };
}

pub fn logger_init(log_config: &LogConfig) -> Result<()> {
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
        let directory = file_path.parent().unwrap_or(Path::new(""));
        let basename = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| log_config.name.as_deref().unwrap_or("alumy"));

        let max_size = log_config.max_size.as_deref()
            .and_then(crate::fs::filesize::parse_size)
            .unwrap_or(1024 * 1024);

        let rolling_appender = BasicRollingFileAppender::new(
            directory.join(format!("{basename}.log")),
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
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.name, None);
        assert_eq!(config.file, None);
        assert_eq!(config.level, None);
    }

    #[test]
    fn test_log_config_new() {
        let config = LogConfig::new(
            Some("test".to_string()),
            Some("test.log".to_string()),
            Some("info".to_string()),
            Some("10M".to_string()),
            Some(5),
        );
        assert_eq!(config.name.as_deref(), Some("test"));
        assert_eq!(config.file.as_deref(), Some("test.log"));
        assert_eq!(config.level.as_deref(), Some("info"));
        assert_eq!(config.max_size.as_deref(), Some("10M"));
        assert_eq!(config.max_files, Some(5));
    }
}
