use std::io;
use std::fmt;

#[cfg(feature = "fmt")]
use super::context::AssertionContext;
use super::result::{AssertionFailure, MatchFailure};

#[cfg(feature = "color")]
use super::color::OutputStyle;

#[derive(Debug, Default)]
struct OutputSegment {
    buf: String,
    #[cfg(feature = "color")]
    style: OutputStyle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OutputStream {
    Stdout,
    Stderr,
}

impl OutputStream {
    pub fn is_stdout(&self) -> bool {
        match self {
            OutputStream::Stdout => true,
            OutputStream::Stderr => false,
        }
    }

    pub fn is_stderr(&self) -> bool {
        match self {
            OutputStream::Stdout => false,
            OutputStream::Stderr => true,
        }
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

#[derive(Debug)]
pub struct FormattedOutput {
    segments: Vec<OutputSegment>,
}

impl FormattedOutput {
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> Self
    where
        Fmt: Format<Value = Value>,
    {
        let mut formatter = Formatter::new();
        format.fmt(&mut formatter, value);
        let mut segments = formatter.prev;
        segments.push(formatter.current);
        Self { segments }
    }

    #[cfg(feature = "color")]
    pub fn print(&self, stream: OutputStream) -> io::Result<()> {
        use std::io::Write;

        use termcolor::{BufferWriter as ColorWriter, WriteColor};

        let writer = match stream {
            OutputStream::Stdout => ColorWriter::stdout(color_choice(stream)),
            OutputStream::Stderr => ColorWriter::stderr(color_choice(stream)),
        };

        let mut buffer = writer.buffer();

        for segment in &self.segments {
            buffer.set_color(&segment.style.into_term())?;
            buffer.write_all(segment.buf.as_bytes())?;
        }

        writer.print(&buffer)
    }

    #[cfg(not(feature = "color"))]
    pub fn print(&self, stream: OutputStream) -> io::Result<()> {
        let mut output: Box<dyn io::Write> = match stream {
            OutputStream::Stdout => Box::new(io::stdout().lock()),
            OutputStream::Stderr => Box::new(io::stderr().lock()),
        };

        for segment in &self.segments {
            output.write_all(segment.buf.as_bytes())?;
        }

        output.flush()
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            f.write_str(&segment.buf)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Formatter {
    prev: Vec<OutputSegment>,
    current: OutputSegment,
}

impl Formatter {
    fn new() -> Self {
        Self {
            prev: Vec::new(),
            current: Default::default(),
        }
    }

    #[cfg(feature = "color")]
    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        let formatted = output.into();
        let new_current = OutputSegment {
            buf: String::new(),
            style: self.current.style.clone(),
        };
        self.prev.push(std::mem::replace(&mut self.current, new_current));
        self.prev.extend(formatted.segments);
    }

    #[cfg(not(feature = "color"))]
    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        let formatted = output.into();
        self.prev.push(std::mem::take(&mut self.current));
        self.prev.extend(formatted.segments);
    }
}

#[cfg(feature = "color")]
impl Formatter {
    pub fn style(&self) -> &OutputStyle {
        &self.current.style
    }

    pub fn set_style(&mut self, style: OutputStyle) {
        self.prev.push(std::mem::replace(&mut self.current, OutputSegment {
            buf: String::new(),
            style,
        }));
    }

    pub fn with_style(&mut self, block: impl FnOnce(&mut OutputStyle)) {
        let mut new_style = self.current.style.clone();
        block(&mut new_style);
        self.set_style(new_style);
    }
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current.buf.push_str(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.current.buf.push(c);
        Ok(())
    }
}

pub trait Format {
    type Value;

    fn fmt(&self, f: &mut Formatter, value: Self::Value);
}

pub trait ResultFormat: Format<Value = MatchFailure<Self::Pos, Self::Neg>> {
    type Pos;
    type Neg;
}

pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    type Context;
}

#[cfg(feature = "fmt")]
#[derive(Debug)]
pub struct DefaultAssertionFormat;

#[cfg(feature = "fmt")]
impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

#[cfg(feature = "fmt")]
impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
}
