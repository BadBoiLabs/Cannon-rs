use crate::syscalls::print;
use alloc::string::ToString;
use log::{Metadata, Record};

pub struct Logger;

pub static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print(&format_args!("{} - {}", record.level(), record.args()).to_string())
                .expect("failed to print log message");
        }
    }

    fn flush(&self) {}
}

#[macro_export]
macro_rules! init_logger {
    ( $x:expr ) => {
        use cannon_io::logger::LOGGER;
        let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level($x));
    };
}

pub use init_logger;
