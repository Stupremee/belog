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

pub fn try_init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = BeautifulLogger { level };
    log::set_max_level(level.to_level_filter());
    log::set_boxed_logger(Box::new(logger))?;
    Ok(())
}

pub fn init_with_level(level: Level) {
    try_init_with_level(level).expect("Failed to set the logger.");
}

pub fn try_init() -> Result<(), SetLoggerError> {
    try_init_with_level(Level::Info)
}

pub fn init() {
    try_init().expect("Failed to set the logger.");
}
