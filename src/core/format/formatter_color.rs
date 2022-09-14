#![cfg(feature = "color")]

use std::fmt;
use std::io::{self, Write};

use super::strings::indent_vec;
use super::{color::color_writer, Format, OutputStream, OutputStyle};

#[derive(Debug, Default)]
struct OutputSegment {
    buf: String,
    style: OutputStyle,
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

    pub fn write_str(&mut self, s: impl AsRef<str>) {
        self.current.buf.push_str(s.as_ref());
    }

    pub fn write_char(&mut self, c: char) {
        self.current.buf.push(c);
    }

    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        let formatted = output.into();
        let new_current = OutputSegment {
            buf: String::new(),
            style: self.current.style.clone(),
        };
        self.prev
            .push(std::mem::replace(&mut self.current, new_current));
        self.prev.extend(formatted.segments);
    }

    pub fn style(&self) -> &OutputStyle {
        &self.current.style
    }

    pub fn set_style(&mut self, style: OutputStyle) {
        self.prev.push(std::mem::replace(
            &mut self.current,
            OutputSegment {
                buf: String::new(),
                style,
            },
        ));
    }

    pub fn reset_style(&mut self) {
        self.set_style(Default::default());
    }
}

#[derive(Debug)]
pub struct FormattedOutput {
    segments: Vec<OutputSegment>,
}

impl FormattedOutput {
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> Result<Self, Fmt::Error>
    where
        Fmt: Format<Value = Value>,
    {
        let mut formatter = Formatter::new();
        format.fmt(&mut formatter, value)?;
        let mut segments = formatter.prev;
        segments.push(formatter.current);
        Ok(Self { segments })
    }

    fn indented_inner(self, spaces: u32, hanging: bool) -> Self {
        let mut styles = Vec::with_capacity(self.segments.len());
        let mut buffers = Vec::with_capacity(self.segments.len());

        for segment in self.segments {
            styles.push(segment.style);
            buffers.push(segment.buf);
        }

        Self {
            segments: indent_vec(buffers, spaces, hanging)
                .into_iter()
                .zip(styles)
                .map(|(buf, style)| OutputSegment { buf, style })
                .collect(),
        }
    }

    pub fn indented(self, spaces: u32) -> Self {
        self.indented_inner(spaces, false)
    }

    pub fn indented_hanging(self, spaces: u32) -> Self {
        self.indented_inner(spaces, true)
    }

    pub fn print(&self, stream: OutputStream) -> io::Result<()> {
        use termcolor::WriteColor;

        let writer = color_writer(stream);
        let mut buffer = writer.buffer();

        for segment in &self.segments {
            buffer.set_color(&segment.style.into_term())?;
            buffer.write_all(segment.buf.as_bytes())?;
        }

        writer.print(&buffer)
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
