// SPDX-License-Identifier: Apache-2.0

// simple colored logger

// TODO: isatty

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::logger::_log($crate::logger::_LogLevel::Error, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::logger::_log($crate::logger::_LogLevel::Warn, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::logger::_log($crate::logger::_LogLevel::Info, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::logger::_log($crate::logger::_LogLevel::Debug, file!(), line!(), format_args!($($arg)*));)
}

#[doc(hidden)]
pub enum _LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

#[doc(hidden)]
pub fn _log(level: _LogLevel, file: &str, line: u32, args: std::fmt::Arguments) {
    match level {
        _LogLevel::Error => eprintln!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Warn => eprintln!("[W] \x1b[33m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Info => eprintln!("[I] \x1b[34m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Debug => eprintln!("[D] \x1b[37m{}:{} {}\x1b[0m", file, line, args),
    }
}

/// Returns [`failure::Error`] along with logging it.
#[macro_export]
macro_rules! ret_e {
    // ref: failure-0.1.8/src/macros.rs bail!
    ($($arg:tt)*) => {
        return Err(crate::_err(file!(), line!(), format_args!($($arg)*)))
    }
}

#[doc(hidden)]
pub(crate) fn _err(file: &str, line: u32, args: std::fmt::Arguments) -> failure::Error {
    // let msg = format!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args);
    // let err = failure::err_msg(msg);
    // TODO: issue: confusing error message:
    // error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
    //   --> src/utils.rs:51:15
    //    |
    // 51 |     let err = failure::err_msg(msg);
    //    |               ^^^^^^^^^^^^^^^^ `core::fmt::Opaque` cannot be shared between threads safely
    //    |
    //   ::: /home/wsh/.cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.8/src/error_message.rs:11:44
    //    |
    // 11 | pub fn err_msg<D: Display + Debug + Sync + Send + 'static>(msg: D) -> Error {
    //    |                                            ---- required by this bound in `failure::error_message::err_msg`
    //    |
    //    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
    //    = note: required because it appears within the type `&core::fmt::Opaque`
    //    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
    //    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
    //    = note: required because of the requirements on the impl of `std::marker::Send` for `&[std::fmt::ArgumentV1<'_>]`
    //    = note: required because it appears within the type `std::fmt::Arguments<'_>`

    let msg = format!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args);
    let err = failure::err_msg(msg);
    eprintln!("{}", err);
    err
}

// // TODO: set logger and test
// //   rust set callback
// static mut log_func: &'static Option(fn(crate::_LogLevel, &str, u32, std::fmt::Arguments)) =
//     &None;
// pub fn set_logger(f: fn(crate::_LogLevel, &str, u32, std::fmt::Arguments)) {
//     log_func = &Some(f);
// }

#[macro_export]
macro_rules! error_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[31m[E] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! warn_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[33m[W] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! info_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[34m[I] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! debug_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[37m[D] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // TODO: set output stream; test by string or stderr
        error!("hello {}", "world");
        warn!("hello {}", "world");
        info!("hello {}", "world");
        debug!("hello {}", "world");
        error_simpler!("hello {}", "world");
        warn_simpler!("hello {}", "world");
        info_simpler!("hello {}", "world");
        debug_simpler!("hello {}", "world");

        assert_eq!(2 + 2, 4);
    }
}
