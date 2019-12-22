//! A logging library which prints a minimal, colored and beautiful output.

#[cfg(feature = "colored")]
extern crate colored;
#[cfg(feature = "colored")]
use colored::*;

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct BeautifulLogger {
    level: Level,
}

impl Log for BeautifulLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let prefix = match record.level() {
            Level::Error => "error",
            Level::Debug => "debug",
            Level::Trace => "trace",
            Level::Warn => "warn",
            Level::Info => "info",
        };

        #[cfg(feature = "colored")]
        let prefix = (match record.level() {
            Level::Error => prefix.red(),
            Level::Debug => prefix.cyan(),
            Level::Trace => prefix.purple(),
            Level::Warn => prefix.yellow(),
            Level::Info => prefix.white(),
        })
        .bold();

        let message = format!("{}: {}", prefix, record.args());
        #[cfg(feature = "colored")]
        let message = message.bold();
        println!("{}", message);
    }

    fn flush(&self) {}
}

/// Tries to initialize the logger with the given level.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate log;
/// # extern crate belog;
/// # fn main() {
/// belog::try_init_with_level(log::Level::Debug).expect("Failed to initialize logger.");
///
/// info!("Some info here");
/// trace!("Some trace message here");
/// # }
/// ```
pub fn try_init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = BeautifulLogger { level };
    log::set_max_level(level.to_level_filter());
    log::set_boxed_logger(Box::new(logger))?;
    Ok(())
}

/// Initializes the logger with the given level,
/// but panics if it fails to set the logger.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate log;
/// # extern crate belog;
/// # fn main() {
/// belog::init_with_level(log::Level::Info);
///
/// info!("Some info here");
/// debug!("Some debug message here");
/// # }
/// ```
pub fn init_with_level(level: Level) {
    try_init_with_level(level).expect("Failed to set the logger.");
}

/// Tries to initialize the logger with the default level, which is set to Info.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate log;
/// # extern crate belog;
/// # fn main() {
/// belog::try_init().expect("Failed to initialize logger.");
///
/// info!("Some info here");
/// error!("An error occurred");
/// # }
/// ```
pub fn try_init() -> Result<(), SetLoggerError> {
    try_init_with_level(Level::Info)
}

/// Initializes the logger with the default level, which is set to Info,
/// but panics if it fails.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate log;
/// # extern crate belog;
/// # fn main() {
/// belog::init();
///
/// info!("Some info here");
/// error!("An error occurred");
/// # }
/// ```
pub fn init() {
    try_init().expect("Failed to set the logger.");
}
