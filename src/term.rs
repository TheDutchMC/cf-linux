use std::fmt::Arguments;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[macro_export]
macro_rules! log {
    ($color:expr, $($arg:tt)+) => ({
        $crate::term::termcolor($crate::__log_format_args!($($arg)+), $color);
    });
}

#[macro_export]
macro_rules! white {
    ($($arg:tt)+) => {
        $crate::log!(::termcolor::Color::White, $($arg)+);
    }
}

#[macro_export]
macro_rules! red {
    ($($arg:tt)+) => {
        $crate::log!(::termcolor::Color::Red, $($arg)+);
    }
}

#[macro_export]
macro_rules! yellow {
    ($($arg:tt)+) => {
        $crate::log!(::termcolor::Color::Yellow, $($arg)+);
    }
}

pub fn termcolor(msg: Arguments<'_>, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("Failed to set color");
    writeln!(&mut stdout, "{}", msg).expect("Failed to writeln");
    stdout.set_color(ColorSpec::new().set_fg(None)).expect("Failed to set color");
}

#[cfg(test)]
mod test {

    #[test]
    fn white() {
        white!("White text!");
    }

    #[test]
    fn red() {
        red!("Red {}", "text");
    }

    #[test]
    fn yellow() {
        let text = "text";
        yellow!("Yellow {:?}", text);
    }
}