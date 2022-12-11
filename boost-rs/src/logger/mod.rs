//! Logger utils for `log` crate.
//!
//! Init a global logger for `log` crate.
//!
//! # Usage
//!
//! ```
//! # fn main() {
//! use log::{debug, error, info, trace, warn};
//! use boost_rs::logger;
//!
//! // Init the global logger
//! logger::init(Some(logger::LogLevel::Trace));
//!
//! // Print log
//! trace!("trace");
//! debug!("debug");
//! info!("info");
//! warn!("warn");
//! error!("error");
//! # }
//! ```
//!
pub extern crate log;

use std::env;

use log::{Level, LevelFilter, Log, Metadata, Record};

struct Logger;

/// Logger level, Default: Info
static LOG_LEVEL: &str = "LOG_LEVEL";

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s {
            "Trace" => LogLevel::Trace,
            "Debug" => LogLevel::Debug,
            "Info" => LogLevel::Info,
            "Warn" => LogLevel::Warn,
            "Error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

/// Init logger
///
/// # Usage
///
/// ```
/// # fn main() {
///   // init logger
///   use log::info;
///   boost_rs::logger::init(None);
///   info!("test: {}", "info");
/// # }
/// ```
pub fn init(level: Option<LogLevel>) {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();

    let log_level: LogLevel = get_log_level(level);
    log::set_max_level(match log_level {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    });
}

fn get_log_level(level: Option<LogLevel>) -> LogLevel {
    match level {
        Some(level) => level,
        None => env::var(LOG_LEVEL)
            .unwrap_or_else(|_| "Info".to_string())
            .as_str()
            .into(),
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };

        println!(
            "\u{1B}[{}m[{:>5}]: {} - {}\u{1B}[0m",
            color,
            record.level(),
            record.target(),
            record.args(),
        );
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use log::{trace, warn};

    use crate::logger;
    use crate::logger::LogLevel::Warn;

    #[test]
    fn test_init_warn() {
        logger::init(Some(Warn));
        trace!("cannot see trace");
        warn!("can see warning");
        assert!(true)
    }
}
