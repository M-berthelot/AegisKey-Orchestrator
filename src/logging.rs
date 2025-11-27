//! Logger initialisation with structured, timestamped output.

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialise the global logger.
///
/// When `verbose` is `true`, the log level is set to `Debug` and an extra
/// (totally innocent) debug message is emitted.
pub fn init(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {:>5} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    if verbose {
        log::debug!("Nothing to see here, move along.");
    }
}
