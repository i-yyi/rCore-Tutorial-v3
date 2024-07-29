use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! log {
    (error, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            "\x1b[31mERROR: {}\x1b[0m\n",
            format_args!($fmt $(, $($arg)+)?)
        ));
    };

    (warning, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            "\x1b[93mWARNING: {}\x1b[0m\n",
            format_args!($fmt $(, $($arg)+)?)
        ));
    };

    (info, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            "\x1b[34mINFO: {}\x1b[0m\n",
            format_args!($fmt $(, $($arg)+)?)
        ));
    };

    (debug, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            "\x1b[32mDEBUG: {}\x1b[0m\n",
            format_args!($fmt $(, $($arg)+)?)
        ));
    };

    (trace, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
            "\x1b[90mTRACE: {}\x1b[0m\n",
            format_args!($fmt $(, $($arg)+)?)
        ));
    };
}