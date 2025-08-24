use std::path::Path;
use tracing::{Level, level_filters::LevelFilter};
use tracing_appender::{non_blocking::NonBlockingBuilder, rolling};
use tracing_subscriber::{Layer as _, Registry, filter, fmt::Layer, layer::SubscriberExt};

pub use tracing_appender::non_blocking::WorkerGuard;

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

pub fn setup_logger(
    rolling: bool,
    directory: impl AsRef<Path>,
    file_name_prefix: impl AsRef<Path>,
    console_level: &str,
    file_level: &str,
    file_enabled: bool,
) -> (WorkerGuard, WorkerGuard) {
    // Setup log level filter
    let console_level = parse_filter(console_level);
    let file_level = parse_filter(file_level);

    let filter = filter::Targets::new()
        .with_target("sqlx", Level::WARN)
        .with_target("tokio", Level::WARN)
        .with_target("runtime", Level::WARN);

    // Create file appender
    let appender = if rolling {
        rolling::daily(directory, file_name_prefix)
    } else {
        rolling::never(directory, file_name_prefix)
    };

    let (nb_file, guard_file) = NonBlockingBuilder::default()
        .lossy(true)
        .thread_name("Log writer thread")
        .buffered_lines_limit(8192)
        .finish(appender);

    // Create stdout layer
    let (nb_stdout, guard_stdout) = NonBlockingBuilder::default()
        .lossy(true)
        .thread_name("Stdout log writer thread")
        .buffered_lines_limit(8192)
        .finish(std::io::stdout());

    let stdout_layer = Layer::default()
        .with_writer(nb_stdout)
        .with_filter(filter.clone().with_default(console_level));

    #[cfg(feature = "tokio_tracing")]
    let console_layer = console_subscriber::spawn();

    let subscriber = Registry::default().with(stdout_layer);

    #[cfg(feature = "tokio_tracing")]
    let subscriber = subscriber.with(console_layer);

    if file_enabled {
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
