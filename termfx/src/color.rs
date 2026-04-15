pub const OPEN: &str = "\x1b[";
pub const SEPARATE: &str = ";";
pub const CLOSE: &str = "m";

pub const BOLD_CODE: u8 = 1;
pub const UNDERLINE_CODE: u8 = 4;
pub const FRAMED_CODE: u8 = 51;
pub const CIRCLED_CODE: u8 = 52;
pub const OVERLINED_CODE: u8 = 53;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorType {
    Black(bool),
    Red(bool),
    Green(bool),
    Yellow(bool),
    Blue(bool),
    Purple(bool),
    Cyan(bool),
    White(bool),
    Rgb([u8; 3]),
}

impl ColorType {
    const BRIGHT_OFFSET: u8 = 60;
    const BG_OFFSET: u8 = 10;

    fn base_code(&self) -> u8 {
        match self {
            Self::Black(_) => 30,
            Self::Red(_) => 31,
            Self::Green(_) => 32,
            Self::Yellow(_) => 33,
            Self::Blue(_) => 34,
            Self::Purple(_) => 35,
            Self::Cyan(_) => 36,
            Self::White(_) => 37,
            Self::Rgb(_) => 38,
        }
    }

    fn is_bright(&self) -> bool {
        matches!(
            self,
            Self::Black(true)
                | Self::Red(true)
                | Self::Green(true)
                | Self::Yellow(true)
                | Self::Blue(true)
                | Self::Purple(true)
                | Self::Cyan(true)
                | Self::White(true)
        )
    }

    fn code(&self, is_background: bool) -> u8 {
        let mut code = self.base_code();
        if self.is_bright() {
            code += Self::BRIGHT_OFFSET;
        }
        if is_background {
            code += Self::BG_OFFSET;
        }
        code
    }

    fn rgb(&self) -> Option<&[u8; 3]> {
        match self {
            Self::Rgb(rgb) => Some(rgb),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Style {
    flags: u8,
    background: Option<ColorType>,
    foreground: Option<ColorType>,
}

impl Style {
    const BOLD_FLAG: u8 = 1;
    const UNDERLINE_FLAG: u8 = 2;
    const FRAMED_FLAG: u8 = 4;
    const CIRCLED_FLAG: u8 = 8;
    const OVERLINED_FLAG: u8 = 16;

    pub const fn new() -> Self {
        Self {
            flags: 0,
            background: None,
            foreground: None,
        }
    }

    #[inline(always)]
    pub const fn reset() -> Self {
        Self::new()
    }

    pub const fn bold(&mut self) -> &mut Self {
        self.flags |= Self::BOLD_FLAG;
        self
    }
    pub const fn underline(&mut self) -> &mut Self {
        self.flags |= Self::UNDERLINE_FLAG;
        self
    }
    pub const fn framed(&mut self) -> &mut Self {
        self.flags |= Self::FRAMED_FLAG;
        self
    }
    pub const fn circled(&mut self) -> &mut Self {
        self.flags |= Self::CIRCLED_FLAG;
        self
    }
    pub const fn overline(&mut self) -> &mut Self {
        self.flags |= Self::OVERLINED_FLAG;
        self
    }

    pub const fn background(&mut self, color: ColorType) -> &mut Self {
        self.background = Some(color);
        self
    }
    pub const fn foreground(&mut self, color: ColorType) -> &mut Self {
        self.foreground = Some(color);
        self
    }

    pub const fn black(&mut self) -> &mut Self {
        self.foreground(ColorType::Black(false))
    }
    pub const fn red(&mut self) -> &mut Self {
        self.foreground(ColorType::Red(false))
    }
    pub const fn green(&mut self) -> &mut Self {
        self.foreground(ColorType::Green(false))
    }
    pub const fn yellow(&mut self) -> &mut Self {
        self.foreground(ColorType::Yellow(false))
    }
    pub const fn blue(&mut self) -> &mut Self {
        self.foreground(ColorType::Blue(false))
    }
    pub const fn purple(&mut self) -> &mut Self {
        self.foreground(ColorType::Purple(false))
    }
    pub const fn cyan(&mut self) -> &mut Self {
        self.foreground(ColorType::Cyan(false))
    }
    pub const fn white(&mut self) -> &mut Self {
        self.foreground(ColorType::White(false))
    }
    pub const fn bright_black(&mut self) -> &mut Self {
        self.foreground(ColorType::Black(true))
    }
    pub const fn bright_red(&mut self) -> &mut Self {
        self.foreground(ColorType::Red(true))
    }
    pub const fn bright_green(&mut self) -> &mut Self {
        self.foreground(ColorType::Green(true))
    }
    pub const fn bright_yellow(&mut self) -> &mut Self {
        self.foreground(ColorType::Yellow(true))
    }
    pub const fn bright_blue(&mut self) -> &mut Self {
        self.foreground(ColorType::Blue(true))
    }
    pub const fn bright_purple(&mut self) -> &mut Self {
        self.foreground(ColorType::Purple(true))
    }
    pub const fn bright_cyan(&mut self) -> &mut Self {
        self.foreground(ColorType::Cyan(true))
    }
    pub const fn bright_white(&mut self) -> &mut Self {
        self.foreground(ColorType::White(true))
    }
    pub const fn rgb(&mut self, rgb: [u8; 3]) -> &mut Self {
        self.foreground(ColorType::Rgb(rgb))
    }
    pub const fn bg_black(&mut self) -> &mut Self {
        self.background(ColorType::Black(false))
    }
    pub const fn bg_red(&mut self) -> &mut Self {
        self.background(ColorType::Red(false))
    }
    pub const fn bg_green(&mut self) -> &mut Self {
        self.background(ColorType::Green(false))
    }
    pub const fn bg_yellow(&mut self) -> &mut Self {
        self.background(ColorType::Yellow(false))
    }
    pub const fn bg_blue(&mut self) -> &mut Self {
        self.background(ColorType::Blue(false))
    }
    pub const fn bg_purple(&mut self) -> &mut Self {
        self.background(ColorType::Purple(false))
    }
    pub const fn bg_cyan(&mut self) -> &mut Self {
        self.background(ColorType::Cyan(false))
    }
    pub const fn bg_white(&mut self) -> &mut Self {
        self.background(ColorType::White(false))
    }
    pub const fn bg_bright_black(&mut self) -> &mut Self {
        self.background(ColorType::Black(true))
    }
    pub const fn bg_bright_red(&mut self) -> &mut Self {
        self.background(ColorType::Red(true))
    }
    pub const fn bg_bright_green(&mut self) -> &mut Self {
        self.background(ColorType::Green(true))
    }
    pub const fn bg_bright_yellow(&mut self) -> &mut Self {
        self.background(ColorType::Yellow(true))
    }
    pub const fn bg_bright_blue(&mut self) -> &mut Self {
        self.background(ColorType::Blue(true))
    }
    pub const fn bg_bright_purple(&mut self) -> &mut Self {
        self.background(ColorType::Purple(true))
    }
    pub const fn bg_bright_cyan(&mut self) -> &mut Self {
        self.background(ColorType::Cyan(true))
    }
    pub const fn bg_bright_white(&mut self) -> &mut Self {
        self.background(ColorType::White(true))
    }
    pub const fn bg_rgb(&mut self, rgb: [u8; 3]) -> &mut Self {
        self.background(ColorType::Rgb(rgb))
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(OPEN)?;

        let mut has_prev = false;
        for (flag, code) in [
            (Style::BOLD_FLAG, BOLD_CODE),
            (Style::UNDERLINE_FLAG, UNDERLINE_CODE),
            (Style::FRAMED_FLAG, FRAMED_CODE),
            (Style::CIRCLED_FLAG, CIRCLED_CODE),
            (Style::OVERLINED_FLAG, OVERLINED_CODE),
        ] {
            if (self.flags & flag) != 0 {
                if has_prev {
                    f.write_str(SEPARATE)?;
                }
                write!(f, "{code}")?;
                has_prev = true;
            }
        }

        for (color, is_background) in [(self.background, true), (self.foreground, false)] {
            let Some(color) = color else { continue };
            if has_prev {
                f.write_str(SEPARATE)?;
            }
            write!(f, "{}", color.code(is_background))?;
            if let Some(rgb) = color.rgb() {
                for x in rgb {
                    f.write_str(SEPARATE)?;
                    write!(f, "{x}")?;
                }
            }
            has_prev = true;
        }

        if !has_prev {
            f.write_str("0")?;
        }

        f.write_str(CLOSE)
    }
}

pub const RESET: Style = Style::reset();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        const STYLE1: Style = *Style::new().bold().red();
        const STYLE2: Style = *Style::new().bold().bright_cyan().underline();
        println!("{STYLE1}apple{RESET} {STYLE2}orange{RESET}");
    }
}
