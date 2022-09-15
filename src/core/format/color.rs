use bitflags::bitflags;

#[cfg(feature = "color")]
use {super::OutputStream, termcolor::BufferWriter as ColorWriter};

bitflags! {
    #[derive(Default)]
    pub struct TextStyle: u32 {
        const BOLD = 1 << 0;
        const INTENSE = 1 << 1;
        const UNDERLINE = 1 << 2;
        const DIMMED = 1 << 3;
        const ITALIC = 1 << 4;
    }
}

impl TextStyle {
    pub fn reset(&mut self) {
        *self &= TextStyle::empty();
    }

    #[cfg(feature = "color")]
    fn into_term(&self, spec: &mut termcolor::ColorSpec) {
        spec.set_bold(self.contains(Self::BOLD));
        spec.set_intense(self.contains(Self::INTENSE));
        spec.set_underline(self.contains(Self::UNDERLINE));
        spec.set_dimmed(self.contains(Self::DIMMED));
        spec.set_italic(self.contains(Self::ITALIC));
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl TerminalColor {
    #[cfg(feature = "color")]
    fn into_term(&self) -> termcolor::Color {
        use termcolor::Color;

        match self {
            Self::Black => Color::Black,
            Self::Red => Color::Red,
            Self::Green => Color::Green,
            Self::Yellow => Color::Yellow,
            Self::Blue => Color::Blue,
            Self::Magenta => Color::Magenta,
            Self::Cyan => Color::Cyan,
            Self::White => Color::White,
            Self::Ansi256(byte) => Color::Ansi256(*byte),
            Self::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TextColor {
    pub fg: Option<TerminalColor>,
    pub bg: Option<TerminalColor>,
}

impl TextColor {
    pub fn reset(&mut self) {
        self.fg = None;
        self.bg = None;
    }

    #[cfg(feature = "color")]
    fn into_term(&self, spec: &mut termcolor::ColorSpec) {
        match &self.fg {
            Some(color) => spec.set_fg(Some(color.into_term())),
            None => spec.set_fg(None),
        };

        match self.bg {
            Some(color) => spec.set_bg(Some(color.into_term())),
            None => spec.set_bg(None),
        };
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OutputStyle {
    pub style: TextStyle,
    pub color: TextColor,
}

impl OutputStyle {
    pub fn reset(&mut self) {
        self.style.reset();
        self.color.reset();
    }

    #[cfg(feature = "color")]
    pub(super) fn into_term(&self) -> termcolor::ColorSpec {
        let mut spec = termcolor::ColorSpec::new();

        self.style.into_term(&mut spec);
        self.color.into_term(&mut spec);

        spec
    }
}

#[cfg(feature = "color")]
fn color_choice(stream: OutputStream) -> termcolor::ColorChoice {
    let atty_stream = match stream {
        OutputStream::Stdout => atty::Stream::Stdout,
        OutputStream::Stderr => atty::Stream::Stderr,
    };

    if atty::is(atty_stream) {
        termcolor::ColorChoice::Auto
    } else {
        termcolor::ColorChoice::Never
    }
}

#[cfg(feature = "color")]
pub(super) fn color_writer(stream: OutputStream) -> ColorWriter {
    match stream {
        OutputStream::Stdout => ColorWriter::stdout(color_choice(stream)),
        OutputStream::Stderr => ColorWriter::stderr(color_choice(stream)),
    }
}
