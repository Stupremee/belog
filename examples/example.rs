#[macro_use]
extern crate log;
extern crate belog;

fn main() {
    // Initializes the logger with the max_level set to info.
    belog::init_with_level(log::Level::Trace);

    info!("some info");
    warn!("some warn log");
    error!("some error log");
    debug!("some debug log");
    trace!("some trace log");
}
