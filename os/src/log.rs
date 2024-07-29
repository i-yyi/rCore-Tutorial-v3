pub const COLOR_LOG_ERROR_COLOR: &str = "31";
pub const COLOR_LOG_WARN_COLOR: &str = "93";
pub const COLOR_LOG_INFO_COLOR: &str = "34";
pub const COLOR_LOG_DEBUG_COLOR: &str = "32";
pub const COLOR_LOG_TRACE_COLOR: &str = "90";

#[macro_export]
macro_rules! LOG {
    ($level: literal, $color:expr, $fmt:literal $(, $($arg:tt)*)?) => {
        $crate::console::print(format_args!(
            "\x1b[{}m[{}]: {}\x1b[0m\n", 
            $color,
            $level,
            format_args!($fmt $(, $($arg)*)?)
        ));
    };
}

#[macro_export]
macro_rules! ERROR {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::LOG!("ERROR", $crate::log::COLOR_LOG_ERROR_COLOR, $fmt $(, $($arg)*)?);  // 红色
    };
}

#[macro_export]
macro_rules! WARN {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::LOG!("WARN", $crate::log::COLOR_LOG_WARN_COLOR, $fmt $(, $($arg)*)?);  // 红色
    };
}


#[macro_export]
macro_rules! INFO {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::LOG!("INFO", $crate::log::COLOR_LOG_INFO_COLOR, $fmt $(, $($arg)*)?);  // 绿色
    };
}

#[macro_export]
macro_rules! DEBUG {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::LOG!("DEBUG", $crate::log::COLOR_LOG_DEBUG_COLOR, $fmt $(, $($arg)*)?);  // 绿色
    };
}

#[macro_export]
macro_rules! TRACE {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::LOG!("TRACE", $crate::log::COLOR_LOG_TRACE_COLOR, $fmt $(, $($arg)*)?);  // 绿色
    };
}

