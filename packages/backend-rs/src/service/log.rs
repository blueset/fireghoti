use crate::config::CONFIG;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[crate::export(js_name = "initializeRustLogger")]
pub fn initialize_logger() {
    let mut builder = FmtSubscriber::builder();

    // Deprecated
    if let Some(levels) = &CONFIG.log_level {
        if levels.contains(&"error".to_string()) {
            builder = builder.with_max_level(Level::ERROR);
        }
        if levels.contains(&"warning".to_string()) {
            builder = builder.with_max_level(Level::WARN);
        }
        if levels.contains(&"info".to_string()) {
            builder = builder.with_max_level(Level::INFO);
        }
        if levels.contains(&"debug".to_string()) {
            builder = builder.with_max_level(Level::DEBUG);
        }
        if levels.contains(&"trace".to_string()) {
            builder = builder.with_max_level(Level::TRACE);
        }
    } else if let Some(max_level) = &CONFIG.max_log_level {
        builder = builder.with_max_level(match max_level.as_str() {
            "error" => Level::ERROR,
            "warning" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        });
    } else {
        builder = builder.with_max_level(Level::INFO);
    };

    let subscriber = builder.with_level(true).pretty().finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize the logger");
}
