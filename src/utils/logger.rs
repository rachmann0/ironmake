use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// The logger struct
pub struct Logger {
    file: File,
    level: LogLevel,
}

impl Logger {
    /// Creates a new logger writing to a file with a minimum log level
    pub fn new(path: &str, level: LogLevel) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Logger { file, level })
    }

    /// Formats a timestamp in seconds.millis since UNIX_EPOCH
    fn timestamp() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let secs = now.as_secs();
        let millis = now.subsec_millis();
        format!("{}.{:03}", secs, millis)
    }

    /// Internal log function
    fn log(&mut self, level: LogLevel, msg: &str) {
        if level <= self.level {
            let line = format!("[{:?}] {}: {}\n", level, Self::timestamp(), msg);
            let _ = self.file.write_all(line.as_bytes());
        }
    }

    pub fn error(&mut self, msg: &str) { self.log(LogLevel::Error, msg); }
    pub fn warn(&mut self, msg: &str) { self.log(LogLevel::Warn, msg); }
    pub fn info(&mut self, msg: &str) { self.log(LogLevel::Info, msg); }
    pub fn debug(&mut self, msg: &str) { self.log(LogLevel::Debug, msg); }
    pub fn trace(&mut self, msg: &str) { self.log(LogLevel::Trace, msg); }
}

/// Global singleton logger
static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

/// Initializes the global logger (call once in main)
pub fn init_logger(path: &str, level: LogLevel) -> &'static Mutex<Logger> {
    LOGGER.get_or_init(|| {
        let logger = Logger::new(path, level).expect("Failed to create logger");
        Mutex::new(logger)
    })
}

/// Gets a reference to the global logger (must call init_logger first)
pub fn global_logger() -> &'static Mutex<Logger> {
    LOGGER.get().expect("Logger not initialized")
}