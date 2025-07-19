use std::path::Path;
use tracing::level_filters::LevelFilter;
use tracing_appender::{
    non_blocking::{NonBlockingBuilder, WorkerGuard},
    rolling,
};
use tracing_subscriber::{Layer as _, Registry, fmt::Layer, layer::SubscriberExt};

pub fn setup_logger(
    rolling: bool,
    directory: impl AsRef<Path>,
    file_name_prefix: impl AsRef<Path>,
    level: &str,
    file_enabled: bool,
) -> WorkerGuard {
    let appender = if rolling {
        rolling::daily(directory, file_name_prefix)
    } else {
        rolling::never(directory, file_name_prefix)
    };

    let (nb, guard) = NonBlockingBuilder::default()
        .lossy(true)
        .thread_name("Log writer thread")
        .buffered_lines_limit(8192)
        .finish(appender);

    let log_level = match level {
        "error" => LevelFilter::ERROR,
        "warn" => LevelFilter::WARN,
        "info" => LevelFilter::INFO,
        "debug" => LevelFilter::DEBUG,
        "trace" => LevelFilter::TRACE,
        _ => LevelFilter::INFO,
    };

    let stdout_layer = Layer::default()
        .with_writer(std::io::stdout)
        .with_filter(log_level);

    let subscriber = Registry::default().with(stdout_layer);

    if file_enabled {
        let subscriber = subscriber.with(
            Layer::default()
                .with_writer(nb)
                .with_ansi(false)
                .with_filter(log_level),
        );
        tracing::subscriber::set_global_default(subscriber)
    } else {
        tracing::subscriber::set_global_default(subscriber)
    }
    .expect("failed to set global subscriber");

    guard
}
