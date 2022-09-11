use std::io;
use std::fmt;

#[cfg(feature = "fmt")]
use super::context::AssertionContext;
use super::result::{AssertionFailure, MatchFailure};

#[cfg(feature = "color")]
use super::color::OutputStyle;

#[derive(Debug, Default)]
struct OutputSegment {
    buf: Vec<u8>,
    #[cfg(feature = "color")]
    style: OutputStyle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OutputStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Default)]
pub struct Formatter {
    prev: Vec<OutputSegment>,
    current: OutputSegment,
}

#[derive(Debug)]
pub struct FormatReader(Formatter);

impl FormatReader {
    pub(super) fn new(fmt: Formatter) -> Self {
        Self(fmt)
    }
}

impl Formatter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn copy(&mut self, reader: impl Into<FormatReader>) {
        let fmt = reader.into();
        self.prev.push(std::mem::replace(&mut self.current, fmt.0.current));
        self.prev.extend(fmt.0.prev);
    }

    #[cfg(feature = "color")]
    pub fn write_to(&self, stream: OutputStream) -> io::Result<()> {
        use std::io::Write;

        use super::color::color_choice;
        use termcolor::{BufferWriter as ColorWriter, WriteColor};

        let writer = match stream {
            OutputStream::Stdout => ColorWriter::stdout(color_choice()),
            OutputStream::Stderr => ColorWriter::stderr(color_choice()),
        };

        let mut buffer = writer.buffer();

        for segment in &self.prev {
            buffer.set_color(&segment.style.into_term())?;
            buffer.write_all(&segment.buf)?;
        }

        buffer.set_color(&self.current.style.into_term())?;
        buffer.write_all(&self.current.buf)?;

        writer.print(&buffer)
    }

    #[cfg(not(feature = "color"))]
    pub fn write_to(&self, stream: OutputStream) -> io::Result<()> {
        let mut output: Box<dyn io::Write> = match stream {
            OutputStream::Stdout => Box::new(io::stdout().lock()),
            OutputStream::Stderr => Box::new(io::stderr().lock()),
        };

        for segment in &self.prev {
            output.write_all(&segment.buf)?;
        }

        output.write_all(&self.current.buf)?;

        output.flush()
    }
}

#[cfg(feature = "color")]
impl Formatter {
    pub fn style(&self) -> &OutputStyle {
        &self.current.style
    }

    pub fn set_style(&mut self, style: OutputStyle) {
        self.prev.push(std::mem::replace(&mut self.current, OutputSegment {
            buf: Vec::new(),
            style,
        }));
    }

    pub fn with_style(&mut self, block: impl FnOnce(&mut OutputStyle)) {
        let mut new_style = self.current.style.clone();
        block(&mut new_style);
        self.set_style(new_style);
    }
}

impl io::Write for Formatter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.current.buf.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.current.buf.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.current.buf.write_all(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.current.buf.flush()
    }
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current.buf.extend(s.bytes());
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut buf = [0u8; std::mem::size_of::<char>()];
        let encoded = c.encode_utf8(&mut buf);
        self.current.buf.extend(encoded.as_bytes());
        Ok(())
    }
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.prev {
            f.write_str(String::from_utf8_lossy(&segment.buf).as_ref())?;
        }

        f.write_str(String::from_utf8_lossy(&self.current.buf).as_ref())
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
