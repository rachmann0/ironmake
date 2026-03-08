use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[allow(dead_code)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// The logger struct
pub struct Logger {
    file: Option<File>,
    pub level: LogLevel,
}

impl Logger {
    /// Creates a new logger writing to a file with a minimum log level
    pub fn new(path: &str, level: LogLevel) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Logger { file:Some(file), level })
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
            if let Some(file) = &mut self.file {
                let line = format!("[{:?}] {}: {}\n", level, Self::timestamp(), msg);
                let _ = file.write_all(line.as_bytes());
            }
        }
    }

    pub fn error(&mut self, msg: &str) { self.log(LogLevel::Error, msg); }
    pub fn warn(&mut self, msg: &str) { self.log(LogLevel::Warn, msg); }
    pub fn info(&mut self, msg: &str) { self.log(LogLevel::Info, msg); }
    pub fn debug(&mut self, msg: &str) { self.log(LogLevel::Debug, msg); }
    pub fn trace(&mut self, msg: &str) { self.log(LogLevel::Trace, msg); }

    pub fn disabled() -> Self {
        Self {
            level: LogLevel::Off,
            file: None,
        }
    }
}

/// Global singleton logger
static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

/// Initializes the global logger (call once in main)
pub fn init_logger(level: LogLevel) -> &'static Mutex<Logger> {
    LOGGER.get_or_init(|| {
        let logger = if level == LogLevel::Off {
            Logger::disabled()
        } else {
            create_dir_all("logs").expect("Failed to create logs dir");
            let epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Logger::new(&format!("logs/build_{}.log", epoch), level)
                .expect("Failed to create logger")
        };

        Mutex::new(logger)
    })
}

/// Gets a reference to the global logger (must call init_logger first)
pub fn global_logger() -> &'static Mutex<Logger> {
    LOGGER.get().expect("Logger not initialized")
}

// ! MACROS

#[macro_export]
macro_rules! log_error {
// ? ($($arg:tt)*) This means "accept any tokens like println! does",
// ? so you can pass formatted arguments.
    ($($arg:tt)*) => {
        {
            let logger = $crate::utils::logger::global_logger();

            // ? if let and check LogLevel to decrease overhead
            if let Ok(mut logger) = logger.try_lock() {
                if $crate::utils::logger::LogLevel::Error <= logger.level {
                    let msg = format!($($arg)*);
                    logger.error(&msg);
                }
            }
        } // ? logger dropped here → mutex unlocked
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        {
            let logger = $crate::utils::logger::global_logger();

            if let Ok(mut logger) = logger.try_lock() {
                if $crate::utils::logger::LogLevel::Warn <= logger.level {
                    let msg = format!($($arg)*);
                    logger.warn(&msg);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => { 
        {
            let logger = $crate::utils::logger::global_logger();

            if let Ok(mut logger) = logger.try_lock() {
                if $crate::utils::logger::LogLevel::Info <= logger.level {
                    let msg = format!($($arg)*);
                    logger.info(&msg);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        {
            let logger = $crate::utils::logger::global_logger();

            if let Ok(mut logger) = logger.try_lock() {
                if $crate::utils::logger::LogLevel::Debug <= logger.level {
                    let msg = format!($($arg)*);
                    logger.debug(&msg);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        {
            let logger = $crate::utils::logger::global_logger();

            if let Ok(mut logger) = logger.try_lock() {
                if $crate::utils::logger::LogLevel::Trace <= logger.level {
                    let msg = format!($($arg)*);
                    logger.trace(&msg);
                }
            }

        }
    };
}

// ! env variables

pub fn parse_log_level() -> LogLevel {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--log" {
            if let Some(level) = args.next() {
                return match level.as_str() {
                    "off" => LogLevel::Off,
                    "error" => LogLevel::Error,
                    "warn"  => LogLevel::Warn,
                    "info"  => LogLevel::Info,
                    "debug" => LogLevel::Debug,
                    "trace" => LogLevel::Trace,
                    _ => LogLevel::Off,
                };
            }
        }
    }

    LogLevel::Off
}