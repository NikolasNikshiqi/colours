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

// True Colors
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const INDIGO: &str = "\x1b[38;5;63m";
pub const VIOLET: &str = "\x1b[38;5;129m";

#[allow(dead_code)]
pub trait Colour {
    // standard
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;

    // bright
    fn bright_black(&self) -> String;
    fn bright_red(&self) -> String;
    fn bright_green(&self) -> String;
    fn bright_yellow(&self) -> String;
    fn bright_blue(&self) -> String;
    fn bright_magenta(&self) -> String;
    fn bright_cyan(&self) -> String;
    fn bright_white(&self) -> String;

    // true
    fn orange(&self) -> String;
    fn indigo(&self) -> String;
    fn violet(&self) -> String;

    fn truecolor_rgb(&self,r: u8,g: u8,b: u8 ) -> String;
    fn truecolor(&self, code: u8) -> String;

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

    fn orange(&self) -> String {
        format!("{}{}{}", ORANGE, self, RESET)
    }

    fn indigo(&self) -> String {
        format!("{}{}{}", INDIGO, self, RESET)
    }

    fn violet(&self) -> String {
        format!("{}{}{}", VIOLET, self, RESET)
    }

    fn truecolor_rgb(&self,r: u8,g: u8,b: u8 ) -> String {
        let true_colour: String = format!("{};{};{};{}m","\x1b[38;2",r,g,b);
        format!("{}{}{}",true_colour,self,RESET)
    }

    fn truecolor(&self, code: u8) -> String {
        let true_colour: String = format!("{};{}m","\x1b[38;5",code);
        format!("{}{}{}",true_colour,self,RESET)
    }

    fn rainbow(&self) -> String {
        let rainbow_colours = vec![
            BRIGHT_RED,
            ORANGE,
            BRIGHT_YELLOW,
            BRIGHT_GREEN,
            BRIGHT_BLUE,
            INDIGO,
            VIOLET,
        ];
        self.borrow()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let color = rainbow_colours[i % rainbow_colours.len()];
                format!("{}{}{}", color, c, RESET)
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_colors() {
        assert_eq!("test".black(),   format!("{}{}{}", BLACK, "test", RESET));
        assert_eq!("test".red(),     format!("{}{}{}", RED, "test", RESET));
        assert_eq!("test".green(),   format!("{}{}{}", GREEN, "test", RESET));
        assert_eq!("test".yellow(),  format!("{}{}{}", YELLOW, "test", RESET));
        assert_eq!("test".blue(),    format!("{}{}{}", BLUE, "test", RESET));
        assert_eq!("test".magenta(), format!("{}{}{}", MAGENTA, "test", RESET));
        assert_eq!("test".cyan(),    format!("{}{}{}", CYAN, "test", RESET));
        assert_eq!("test".white(),   format!("{}{}{}", WHITE, "test", RESET));
    }

    #[test]
    fn test_bright_colors() {
        assert_eq!("hi".bright_black(),   format!("{}{}{}", BRIGHT_BLACK, "hi", RESET));
        assert_eq!("hi".bright_red(),     format!("{}{}{}", BRIGHT_RED, "hi", RESET));
        assert_eq!("hi".bright_green(),   format!("{}{}{}", BRIGHT_GREEN, "hi", RESET));
        assert_eq!("hi".bright_yellow(),  format!("{}{}{}", BRIGHT_YELLOW, "hi", RESET));
        assert_eq!("hi".bright_blue(),    format!("{}{}{}", BRIGHT_BLUE, "hi", RESET));
        assert_eq!("hi".bright_magenta(), format!("{}{}{}", BRIGHT_MAGENTA, "hi", RESET));
        assert_eq!("hi".bright_cyan(),    format!("{}{}{}", BRIGHT_CYAN, "hi", RESET));
        assert_eq!("hi".bright_white(),   format!("{}{}{}", BRIGHT_WHITE, "hi", RESET));
    }

    #[test]
    fn test_extended_colors() {
        assert_eq!("!".orange(), format!("{}{}{}", ORANGE, "!", RESET));
        assert_eq!("!".indigo(), format!("{}{}{}", INDIGO, "!", RESET));
        assert_eq!("!".violet(), format!("{}{}{}", VIOLET, "!", RESET));
    }

    #[test]
    fn test_truecolor_logic() {
        // Testing 256-color mode (\x1b[38;5;Nm)
        let custom_code = "text".truecolor(150);
        assert_eq!(custom_code, "\x1b[38;5;150mtext\x1b[0m");

        // Testing RGB mode (\x1b[38;2;R;G;Bm)
        let rgb_code = "text".truecolor_rgb(255, 100, 50);
        assert_eq!(rgb_code, "\x1b[38;2;255;100;50mtext\x1b[0m");
    }

    #[test]
    fn test_rainbow() {
        let input = "ABC";
        let result = input.rainbow();
        
        // "A" should be Bright Red, "B" Orange, "C" Bright Yellow
        let expected = format!(
            "{}{}{}{}{}{}{}{}{}",
            BRIGHT_RED, "A", RESET,
            ORANGE, "B", RESET,
            BRIGHT_YELLOW, "C", RESET
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generic_support() {
        // Test that it works on an owned String as well as &str
        let owned = String::from("owned");
        assert_eq!(owned.red(), format!("{}{}{}", RED, "owned", RESET));
    }
}