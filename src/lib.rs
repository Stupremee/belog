//! A logging library which prints a minimal, colored and beautiful output.

#[cfg(feature = "colored")]
extern crate colored;
//#[cfg(feature = "colored")]
//use colored::*;

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct BeautifulLogger {
    level: Level,
}

impl Log for BeautifulLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        unimplemented!()
    }

    fn flush(&self) {
        unimplemented!()
    }
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
    try_init_with_level(Level::Trace)
}

pub fn init() {
    try_init().expect("Failed to set the logger.");
}
