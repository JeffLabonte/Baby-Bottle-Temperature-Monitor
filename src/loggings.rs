use log::{self, Level, Log, SetLoggerError};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        todo!("Not implemented yet")
    }
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logs() -> Result<(), SetLoggerError> {
    Ok(())
}
