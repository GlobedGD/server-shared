use serde::{Deserialize, Serialize};
use tracing::{Level, level_filters::LevelFilter};
use tracing_appender::{
    non_blocking::NonBlockingBuilder,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{Layer as _, Registry, filter, fmt::Layer, layer::SubscriberExt};
use validator::{Validate, ValidationError};

pub use tracing_appender::non_blocking::WorkerGuard;

use crate::config::log_buffer_size_for_memlimit;

fn default_file_enabled() -> bool {
    true
}

fn default_directory() -> String {
    "logs".to_string()
}

fn default_file_level() -> String {
    "debug".to_string()
}

fn default_console_level() -> String {
    "info".to_string()
}

fn default_filename() -> String {
    "server".to_string()
}

fn default_rolling() -> bool {
    false
}

fn default_retention_days() -> u32 {
    7
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoggerConfig {
    /// Whether to enable logging to a file. If disabled, logs will only be printed to stdout.
    #[serde(default = "default_file_enabled")]
    pub file_enabled: bool,
    /// The directory where logs will be stored.
    #[serde(default = "default_directory")]
    pub directory: String,
    /// Minimum log level to print to the file. Logs below this level will be ignored. Possible values: 'trace', 'debug', 'info', 'warn', 'error'.
    #[serde(default = "default_file_level")]
    #[validate(custom(function = "validate_log_level"))]
    pub file_level: String,
    /// Minimum log level to print to the console. Logs below this level will be ignored. Possible values: 'trace', 'debug', 'info', 'warn', 'error'.
    #[serde(default = "default_console_level")]
    #[validate(custom(function = "validate_log_level"))]
    pub console_level: String,
    /// Prefix for the filename of the log file.
    #[serde(default = "default_filename")]
    pub filename: String,
    /// Whether to roll the log file daily. If enabled, rather than overwriting the same log file on restart,
    /// a new log file will be created with the current date appended to the filename.
    #[serde(default = "default_rolling")]
    pub rolling: bool,
    /// If rolling is enabled and this value is nonzero, old logs are deleted after this many days
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            file_enabled: default_file_enabled(),
            directory: default_directory(),
            file_level: default_file_level(),
            console_level: default_console_level(),
            filename: default_filename(),
            rolling: default_rolling(),
            retention_days: default_retention_days(),
        }
    }
}

pub fn setup_logger(config: &LoggerConfig, mem_usage: u32) -> (WorkerGuard, WorkerGuard) {
    // Setup log level filter
    let console_level = parse_filter(&config.console_level);
    let file_level = parse_filter(&config.file_level);

    let filter = filter::Targets::new()
        .with_target("sqlx", Level::WARN)
        .with_target("tokio", Level::WARN)
        .with_target("runtime", Level::WARN)
        .with_target("serenity", Level::WARN)
        .with_target("hickory_resolver", Level::WARN)
        .with_target("hickory_proto", Level::WARN)
        .with_target("h2", Level::WARN)
        .with_target("hyper", Level::WARN);

    // Create file appender
    let appender = RollingFileAppender::builder()
        .rotation(if config.rolling {
            Rotation::DAILY
        } else {
            Rotation::NEVER
        })
        .filename_prefix(&config.filename)
        .max_log_files(config.retention_days as usize)
        .build(&config.directory)
        .expect("failed to build file appender");

    let buf_lines = log_buffer_size_for_memlimit(mem_usage);
    let stdout_buf_lines = buf_lines / 4;

    let (nb_file, guard_file) = NonBlockingBuilder::default()
        .lossy(true)
        .thread_name("Log writer thread")
        .buffered_lines_limit(buf_lines)
        .finish(appender);

    // Create stdout layer
    let (nb_stdout, guard_stdout) = NonBlockingBuilder::default()
        .lossy(true)
        .thread_name("Stdout log writer thread")
        .buffered_lines_limit(stdout_buf_lines)
        .finish(std::io::stdout());

    let stdout_layer = Layer::default()
        .with_writer(nb_stdout)
        .with_filter(filter.clone().with_default(console_level));

    #[cfg(feature = "tokio_tracing")]
    let console_layer = console_subscriber::spawn();

    let subscriber = Registry::default().with(stdout_layer);

    #[cfg(feature = "tokio_tracing")]
    let subscriber = subscriber.with(console_layer);

    if config.file_enabled {
        let file_layer = Layer::default()
            .with_writer(nb_file)
            .with_ansi(false)
            .with_filter(filter.clone().with_default(file_level));

        let subscriber = subscriber.with(file_layer);

        tracing::subscriber::set_global_default(subscriber)
    } else {
        tracing::subscriber::set_global_default(subscriber)
    }
    .expect("failed to set global subscriber");

    (guard_file, guard_stdout)
}

pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload_as_str().unwrap_or("unknown panic payload");
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        tracing::error!("Server panicked at {location}: {payload}");
    }));
}

fn parse_filter(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "error" => LevelFilter::ERROR,
        "warn" => LevelFilter::WARN,
        "info" => LevelFilter::INFO,
        "debug" => LevelFilter::DEBUG,
        "trace" => LevelFilter::TRACE,
        _ => LevelFilter::INFO,
    }
}
fn validate_log_level(level: &str) -> Result<(), ValidationError> {
    match level.to_lowercase().as_str() {
        "trace" | "debug" | "info" | "warn" | "error" => Ok(()),
        _ => Err(ValidationError::new("invalid log level")),
    }
}
