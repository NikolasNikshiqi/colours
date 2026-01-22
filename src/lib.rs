use std::{borrow::Borrow, fmt::Display};

const RESET: &str = "\x1b[0m";
const RESET_BOLD: &str = "\x1b[22m";
const RESET_ITALIC: &str = "\x1b[23m";

// Style
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";

// Standard Colours
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

// Bright Colours
const BRIGHT_BLACK: &str = "\x1b[90m";
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_YELLOW: &str = "\x1b[93m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BRIGHT_MAGENTA: &str = "\x1b[95m";
const BRIGHT_CYAN: &str = "\x1b[96m";
const BRIGHT_WHITE: &str = "\x1b[97m";

// Background colours
const BG_BLACK: &str = "\x1b[40m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";
const BG_WHITE: &str = "\x1b[47m";

const BG_BRIGHT_BLACK: &str = "\x1b[100m";
const BG_BRIGHT_RED: &str = "\x1b[101m";
const BG_BRIGHT_GREEN: &str = "\x1b[102m";
const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
const BG_BRIGHT_BLUE: &str = "\x1b[104m";
const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
const BG_BRIGHT_CYAN: &str = "\x1b[106m";
const BG_BRIGHT_WHITE: &str = "\x1b[107m";

// True Colours
const ORANGE: &str = "\x1b[38;5;208m";
const INDIGO: &str = "\x1b[38;5;63m";
const VIOLET: &str = "\x1b[38;5;129m";

#[allow(dead_code)]
pub trait Colour {
    // style
    fn bold(&self) -> String;
    fn italic(&self) -> String;

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

    // background
    fn bg_black(&self) -> String;
    fn bg_red(&self) -> String;
    fn bg_green(&self) -> String;
    fn bg_yellow(&self) -> String;
    fn bg_blue(&self) -> String;
    fn bg_magenta(&self) -> String;
    fn bg_cyan(&self) -> String;
    fn bg_white(&self) -> String;

    // bright
    fn bg_bright_black(&self) -> String;
    fn bg_bright_red(&self) -> String;
    fn bg_bright_green(&self) -> String;
    fn bg_bright_yellow(&self) -> String;
    fn bg_bright_blue(&self) -> String;
    fn bg_bright_magenta(&self) -> String;
    fn bg_bright_cyan(&self) -> String;
    fn bg_bright_white(&self) -> String;

    // true
    fn orange(&self) -> String;
    fn indigo(&self) -> String;
    fn violet(&self) -> String;

    fn truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn truecolour(&self, code: u8) -> String;

    fn rainbow(&self) -> String;
}
impl<T> Colour for T
where
    T: Borrow<str> + Display,
{
    fn bold(&self) -> String {
        format!("{}{}{}", BOLD, self, RESET_BOLD)
    }

    fn italic(&self) -> String {
        format!("{}{}{}", ITALIC, self, RESET_ITALIC)
    }

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

    // --- Background Standard ---
    fn bg_black(&self) -> String {
        format!("{}{}{}", BG_BLACK, self, RESET)
    }
    fn bg_red(&self) -> String {
        format!("{}{}{}", BG_RED, self, RESET)
    }
    fn bg_green(&self) -> String {
        format!("{}{}{}", BG_GREEN, self, RESET)
    }
    fn bg_yellow(&self) -> String {
        format!("{}{}{}", BG_YELLOW, self, RESET)
    }
    fn bg_blue(&self) -> String {
        format!("{}{}{}", BG_BLUE, self, RESET)
    }
    fn bg_magenta(&self) -> String {
        format!("{}{}{}", BG_MAGENTA, self, RESET)
    }
    fn bg_cyan(&self) -> String {
        format!("{}{}{}", BG_CYAN, self, RESET)
    }
    fn bg_white(&self) -> String {
        format!("{}{}{}", BG_WHITE, self, RESET)
    }

    // --- Background Bright ---
    fn bg_bright_black(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_BLACK, self, RESET)
    }
    fn bg_bright_red(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_RED, self, RESET)
    }
    fn bg_bright_green(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_GREEN, self, RESET)
    }
    fn bg_bright_yellow(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_YELLOW, self, RESET)
    }
    fn bg_bright_blue(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_BLUE, self, RESET)
    }
    fn bg_bright_magenta(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_MAGENTA, self, RESET)
    }
    fn bg_bright_cyan(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_CYAN, self, RESET)
    }
    fn bg_bright_white(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_WHITE, self, RESET)
    }

    // true colours

    fn orange(&self) -> String {
        format!("{}{}{}", ORANGE, self, RESET)
    }

    fn indigo(&self) -> String {
        format!("{}{}{}", INDIGO, self, RESET)
    }

    fn violet(&self) -> String {
        format!("{}{}{}", VIOLET, self, RESET)
    }

    fn truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String {
        let true_colour: String = format!("{};{};{};{}m", "\x1b[38;2", r, g, b);
        format!("{}{}{}", true_colour, self, RESET)
    }

    fn truecolour(&self, code: u8) -> String {
        let true_colour: String = format!("{};{}m", "\x1b[38;5", code);
        format!("{}{}{}", true_colour, self, RESET)
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
    fn test_styles() {
        assert_eq!("test".bold(), format!("{}{}{}", BOLD, "test", RESET_BOLD));
        assert_eq!(
            "test".italic(),
            format!("{}{}{}", ITALIC, "test", RESET_ITALIC)
        );
    }

    #[test]
    fn test_standard_colors() {
        assert_eq!("test".black(), format!("{}{}{}", BLACK, "test", RESET));
        assert_eq!("test".red(), format!("{}{}{}", RED, "test", RESET));
        assert_eq!("test".green(), format!("{}{}{}", GREEN, "test", RESET));
        assert_eq!("test".yellow(), format!("{}{}{}", YELLOW, "test", RESET));
        assert_eq!("test".blue(), format!("{}{}{}", BLUE, "test", RESET));
        assert_eq!("test".magenta(), format!("{}{}{}", MAGENTA, "test", RESET));
        assert_eq!("test".cyan(), format!("{}{}{}", CYAN, "test", RESET));
        assert_eq!("test".white(), format!("{}{}{}", WHITE, "test", RESET));
    }

    #[test]
    fn test_bright_colors() {
        assert_eq!(
            "hi".bright_black(),
            format!("{}{}{}", BRIGHT_BLACK, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_red(),
            format!("{}{}{}", BRIGHT_RED, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_green(),
            format!("{}{}{}", BRIGHT_GREEN, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_yellow(),
            format!("{}{}{}", BRIGHT_YELLOW, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_blue(),
            format!("{}{}{}", BRIGHT_BLUE, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_magenta(),
            format!("{}{}{}", BRIGHT_MAGENTA, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_cyan(),
            format!("{}{}{}", BRIGHT_CYAN, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_white(),
            format!("{}{}{}", BRIGHT_WHITE, "hi", RESET)
        );
    }

    #[test]
    fn test_background_standard() {
        assert_eq!("bg".bg_black(),   format!("{}{}{}", BG_BLACK, "bg", RESET));
        assert_eq!("bg".bg_red(),     format!("{}{}{}", BG_RED, "bg", RESET));
        assert_eq!("bg".bg_green(),   format!("{}{}{}", BG_GREEN, "bg", RESET));
        assert_eq!("bg".bg_yellow(),  format!("{}{}{}", BG_YELLOW, "bg", RESET));
        assert_eq!("bg".bg_blue(),    format!("{}{}{}", BG_BLUE, "bg", RESET));
        assert_eq!("bg".bg_magenta(), format!("{}{}{}", BG_MAGENTA, "bg", RESET));
        assert_eq!("bg".bg_cyan(),    format!("{}{}{}", BG_CYAN, "bg", RESET));
        assert_eq!("bg".bg_white(),   format!("{}{}{}", BG_WHITE, "bg", RESET));
    }

    #[test]
    fn test_background_bright() {
        assert_eq!("bg".bg_bright_black(),   format!("{}{}{}", BG_BRIGHT_BLACK, "bg", RESET));
        assert_eq!("bg".bg_bright_red(),     format!("{}{}{}", BG_BRIGHT_RED, "bg", RESET));
        assert_eq!("bg".bg_bright_green(),   format!("{}{}{}", BG_BRIGHT_GREEN, "bg", RESET));
        assert_eq!("bg".bg_bright_yellow(),  format!("{}{}{}", BG_BRIGHT_YELLOW, "bg", RESET));
        assert_eq!("bg".bg_bright_blue(),    format!("{}{}{}", BG_BRIGHT_BLUE, "bg", RESET));
        assert_eq!("bg".bg_bright_magenta(), format!("{}{}{}", BG_BRIGHT_MAGENTA, "bg", RESET));
        assert_eq!("bg".bg_bright_cyan(),    format!("{}{}{}", BG_BRIGHT_CYAN, "bg", RESET));
        assert_eq!("bg".bg_bright_white(),   format!("{}{}{}", BG_BRIGHT_WHITE, "bg", RESET));
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
        let custom_code = "text".truecolour(150);
        assert_eq!(custom_code, "\x1b[38;5;150mtext\x1b[0m");

        // Testing RGB mode (\x1b[38;2;R;G;Bm)
        let rgb_code = "text".truecolour_rgb(255, 100, 50);
        assert_eq!(rgb_code, "\x1b[38;2;255;100;50mtext\x1b[0m");
    }

    #[test]
    fn test_rainbow() {
        let input = "ABC";
        let result = input.rainbow();

        // "A" should be Bright Red, "B" Orange, "C" Bright Yellow
        let expected = format!(
            "{}{}{}{}{}{}{}{}{}",
            BRIGHT_RED, "A", RESET, ORANGE, "B", RESET, BRIGHT_YELLOW, "C", RESET
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
