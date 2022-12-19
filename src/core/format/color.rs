use bitflags::bitflags;

#[cfg(feature = "color")]
use colored::{ColoredString, Colorize};

bitflags! {
    /// The style of text in a terminal emulator.
    ///
    /// Note that not all styles may be supported on all platforms and by all terminal emulators. A
    /// misconfigured `$TERM` or terminfo database can also cause text styles to not render
    /// properly.
    #[derive(Default)]
    pub struct TextStyle: u32 {
        const BOLD = 1 << 0;
        const UNDERLINE = 1 << 1;
        const DIMMED = 1 << 2;
        const ITALIC = 1 << 3;
        const STRIKETHROUGH = 1 << 4;
        const REVERSED = 1 << 5;
    }
}

impl TextStyle {
    /// Reset this text style back to the default (no style).
    pub fn reset(&mut self) {
        *self &= TextStyle::empty();
    }

    #[cfg(feature = "color")]
    fn apply(&self, s: ColoredString) -> ColoredString {
        let mut output = s;

        if self.contains(Self::BOLD) {
            output = output.bold();
        }

        if self.contains(Self::UNDERLINE) {
            output = output.underline();
        }

        if self.contains(Self::DIMMED) {
            output = output.dimmed();
        }

        if self.contains(Self::ITALIC) {
            output = output.italic();
        }

        if self.contains(Self::STRIKETHROUGH) {
            output = output.strikethrough();
        }

        if self.contains(Self::REVERSED) {
            output = output.reversed();
        }

        output
    }
}

/// A color to display in a terminal emulator.
///
/// Note that [`Color::Rgb`] may not be supported on all platforms or by all terminal emulators.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
}

#[cfg(feature = "color")]
impl Color {
    fn into_color(self) -> colored::Color {
        use colored::Color;

        match self {
            Self::Black => Color::Black,
            Self::Red => Color::Red,
            Self::Green => Color::Green,
            Self::Yellow => Color::Yellow,
            Self::Blue => Color::Blue,
            Self::Magenta => Color::Magenta,
            Self::Cyan => Color::Cyan,
            Self::White => Color::White,
            Self::BrightBlack => Color::BrightBlack,
            Self::BrightRed => Color::BrightRed,
            Self::BrightGreen => Color::BrightGreen,
            Self::BrightYellow => Color::BrightYellow,
            Self::BrightBlue => Color::BrightBlue,
            Self::BrightMagenta => Color::BrightMagenta,
            Self::BrightCyan => Color::BrightCyan,
            Self::BrightWhite => Color::BrightWhite,
            Self::Rgb(r, g, b) => Color::TrueColor { r, g, b },
        }
    }
}

/// The color of text in a terminal emulator.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TextColor {
    /// The foreground color.
    pub fg: Option<Color>,

    /// The background color.
    pub bg: Option<Color>,
}

impl TextColor {
    /// Reset this text color back to the default (no color).
    pub fn reset(&mut self) {
        self.fg = None;
        self.bg = None;
    }

    #[cfg(feature = "color")]
    fn apply(&self, s: ColoredString) -> ColoredString {
        match (&self.fg, &self.bg) {
            (None, None) => s,
            (None, Some(bg)) => s.on_color(bg.into_color()),
            (Some(fg), None) => s.color(fg.into_color()),
            (Some(fg), Some(bg)) => s.color(fg.into_color()).on_color(bg.into_color()),
        }
    }
}

/// The style of text in formatted output.
///
/// You can apply an [`OutputStyle`] to text in a formatter using [`Formatter::set_style`].
///
/// # Examples
///
/// ```
/// use xpct::core::{OutputStyle, TextStyle, TextColor, Color};
///
/// let style = OutputStyle {
///     style: TextStyle::BOLD | TextStyle::UNDERLINE,
///     color: TextColor {
///         fg: Some(Color::BrightRed),
///         bg: None,
///     },
/// };
/// ```
///
/// [`Formatter::set_style`]: crate::core::Formatter::set_style
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OutputStyle {
    pub style: TextStyle,
    pub color: TextColor,
}

impl OutputStyle {
    /// Reset this output style back to the default.
    pub fn reset(&mut self) {
        self.style.reset();
        self.color.reset();
    }

    #[cfg(feature = "color")]
    pub(super) fn apply(&self, s: &str) -> ColoredString {
        self.style.apply(self.color.apply(s.into()))
    }
}
