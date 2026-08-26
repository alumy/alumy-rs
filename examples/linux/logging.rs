use alumy::{info, LogConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    LogConfig::new("linux-example", "info")
        .with_file("target/alumy-linux.log", "1M", 2)
        .with_target(true)
        .init()?;

    info!("hello from linux");
    Ok(())
}
