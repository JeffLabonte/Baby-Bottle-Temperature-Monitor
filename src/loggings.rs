use chrono;
use log::{self, Level, LevelFilter, Log, SetLoggerError};

use crate::helpers::{generate_file_name_with_now_time, write_to_file};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let local_time = chrono::offset::Local::now();

            let message = format!(
                "\n{} : {} - {}",
                local_time.to_rfc3339(),
                record.level(),
                record.args()
            );
            let file_path: String = generate_file_name_with_now_time(".log".to_string());
            write_to_file(file_path, message.to_string());
            println!("{}", message);
        }
    }

    fn flush(&self) {
        todo!("Not implemented yet")
    }
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logs() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
