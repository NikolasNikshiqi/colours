use std::{borrow::Borrow, fmt::Display};

pub const RESET: &str = "\x1b[0m";

// Standard Colors
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

// Bright Colors
pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

#[allow(dead_code)]
pub trait Colour {
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;

    fn bright_black(&self) -> String;
    fn bright_red(&self) -> String;
    fn bright_green(&self) -> String;
    fn bright_yellow(&self) -> String;
    fn bright_blue(&self) -> String;
    fn bright_magenta(&self) -> String;
    fn bright_cyan(&self) -> String;
    fn bright_white(&self) -> String;

    fn rainbow(&self) -> String;
}
impl<T> Colour for T
where
    T: Borrow<str> + Display,
{
    fn black(&self) -> String {
        format!("{}{}{}", BLACK, self, RESET)
    }
    fn red(&self) -> String {
        format!("{}{}{}", RED, self, RESET)
    }

    fn green(&self) -> String {
        format!("{}{}{}", GREEN, self, RESET)
    }

    fn yellow(&self) -> String {
        format!("{}{}{}", YELLOW, self, RESET)
    }

    fn blue(&self) -> String {
        format!("{}{}{}", BLUE, self, RESET)
    }

    fn magenta(&self) -> String {
        format!("{}{}{}", MAGENTA, self, RESET)
    }

    fn cyan(&self) -> String {
        format!("{}{}{}", CYAN, self, RESET)
    }

    fn white(&self) -> String {
        format!("{}{}{}", WHITE, self, RESET)
    }

    fn bright_black(&self) -> String {
        format!("{}{}{}", BRIGHT_BLACK, self, RESET)
    }

    fn bright_red(&self) -> String {
        format!("{}{}{}", BRIGHT_RED, self, RESET)
    }

    fn bright_green(&self) -> String {
        format!("{}{}{}", BRIGHT_GREEN, self, RESET)
    }

    fn bright_yellow(&self) -> String {
        format!("{}{}{}", BRIGHT_YELLOW, self, RESET)
    }

    fn bright_blue(&self) -> String {
        format!("{}{}{}", BRIGHT_BLUE, self, RESET)
    }

    fn bright_magenta(&self) -> String {
        format!("{}{}{}", BRIGHT_MAGENTA, self, RESET)
    }

    fn bright_cyan(&self) -> String {
        format!("{}{}{}", BRIGHT_CYAN, self, RESET)
    }

    fn bright_white(&self) -> String {
        format!("{}{}{}", BRIGHT_WHITE, self, RESET)
    }

    fn rainbow(&self) -> String {
        " ".to_string()
    }
}
