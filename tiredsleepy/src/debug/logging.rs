use std::{
    fmt,
    sync::atomic::{AtomicU8, Ordering},
};

extern crate colored;
use colored::*;

#[derive(Debug)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ostr = format!("[{self:?}]");
        match self {
            LogLevel::Trace => write!(f, "{}", ostr.as_str().bold().black()),
            LogLevel::Debug => write!(f, "{}", ostr.as_str().bold().green()),
            LogLevel::Info => write!(f, "{}", ostr.as_str().bold().blue()),
            LogLevel::Warn => write!(f, "{}", ostr.as_str().bold().yellow()),
            LogLevel::Error => write!(f, "{}", ostr.as_str().bold().red()),
            LogLevel::Fatal => write!(f, "{}", ostr.as_str().bold().black().on_red()),
        }
    }
}

static CURRENT_LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Debug as u8);

pub fn set_log_level(level: LogLevel) {
    CURRENT_LOG_LEVEL.store(level as u8, Ordering::SeqCst);
}

/// Please don't use this directly ;; instead use one of the logging macros such as `trace!` or `warn!`.
#[doc(hidden)]
pub fn _internal_log(level: LogLevel, string: std::string::String) {
    if level as u8 >= CURRENT_LOG_LEVEL.load(Ordering::Relaxed) {
        println!("{}", string);
    }
}

// rust analyzer really doesn't like macros generated with this so we have to manually make them ;;
// #[rustfmt::skip] // for some reason, rustfmt really doesn't like this macro.
// macro_rules! make_log_macro {
//     ($d:tt, $name:ident, $level:expr) => {
//         #[macro_export]
//         macro_rules! $name {
//             () => {{
//                 $crate::_internal_log($crate::$level,
//                                       format!("{}:{} {}",
//                                               file!(),
//                                               line!(),
//                                               $crate::$level));
//             }};
//             ($d($d arg:tt)+) => {{
//                 $crate::_internal_log($crate::$level,
//                                       format!("{}:{} {} {}",
//                                               file!(),
//                                               line!(),
//                                               $crate::$level,
//                                               format!($d($d arg),+)));
//             }};
//         }
//     };
// }

// make_log_macro!($, trace, LogLevel::Trace);
// make_log_macro!($, debug, LogLevel::Debug);
// make_log_macro!($, info, LogLevel::Info);
// make_log_macro!($, warn, LogLevel::Warn);
// make_log_macro!($, error, LogLevel::Error);

#[macro_export]
macro_rules! trace {
    () => {{
        $crate::_internal_log($crate::LogLevel::Trace,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Trace));
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Trace,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Trace,
                                      format!($($arg)+)));
    }};
}

#[macro_export]
macro_rules! debug {
    () => {{
        $crate::_internal_log($crate::LogLevel::Debug,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Debug));
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Debug,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Debug,
                                      format!($($arg)+)));
    }};
}

#[macro_export]
macro_rules! info {
    () => {{
        $crate::_internal_log($crate::LogLevel::Info,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Info));
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Info,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Info,
                                      format!($($arg)+)));
    }};
}

#[macro_export]
macro_rules! warn {
    () => {{
        $crate::_internal_log($crate::LogLevel::Warn,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Warn));
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Warn,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Warn,
                                      format!($($arg)+)));
    }};
}

#[macro_export]
macro_rules! error {
    () => {{
        $crate::_internal_log($crate::LogLevel::Error,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Error));
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Error,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Error,
                                      format!($($arg)+)));
    }};
}

#[macro_export]
macro_rules! fatal {
    () => {{
        $crate::_internal_log($crate::LogLevel::Fatal,
                              format!("{}:{} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Fatal));
        panic!();
    }};
    ($($arg:tt)+) => {{
        $crate::_internal_log($crate::LogLevel::Fatal,
                              format!("{}:{} {} {}",
                                      file!(),
                                      line!(),
                                      $crate::LogLevel::Fatal,
                                      format!($($arg)+)));
        panic!();
    }};
}
