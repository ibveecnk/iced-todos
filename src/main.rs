//! A simple todo list application. It allows adding and removing todos.

/// The `Todos` struct and its implementation.
mod todos;
use anyhow::bail;
use todos::Todos;

/// Sets up logging for the application.
///
/// Uses log4rs to configure logging to both stdout and a file.
fn setup_logging() -> anyhow::Result<()> {
    use log4rs::config::Appender;
    use log4rs::filter::threshold::ThresholdFilter;
    use log4rs::Config;

    /// The name/pathname of the log file.
    const LOG_FILE: &str = "todos.log";

    #[cfg(debug_assertions)]
    let console_level_filter = Box::new(ThresholdFilter::new(log::LevelFilter::Debug)); // Trace is too verbose for debug builds
    #[cfg(not(debug_assertions))]
    let console_level_filter = Box::new(ThresholdFilter::new(log::LevelFilter::Info));

    let console_appender = Appender::builder().filter(console_level_filter).build(
        "stdout",
        Box::new(
            log4rs::append::console::ConsoleAppender::builder()
                .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                    "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {f}:{L} {m}{n})}",
                )))
                .build(),
        ),
    );

    let file_appender = Appender::builder().build(
        "file",
        Box::new(
            log4rs::append::file::FileAppender::builder()
                .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                    "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
                )))
                .build(LOG_FILE)?,
        ),
    );

    let config = Config::builder()
        .appender(console_appender)
        .appender(file_appender)
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .appender("file")
                .build(log::LevelFilter::Trace),
        );

    match config {
        Ok(cfg) => {
            let _handle = log4rs::init_config(cfg)?;
            Ok(())
        }
        Err(e) => bail!(e),
    }
}

fn main() -> anyhow::Result<()> {
    setup_logging()?;
    iced::program(Todos::title, Todos::update, Todos::view)
        .centered()
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()?;
    Ok(())
}
