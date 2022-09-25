#![cfg(feature = "color")]

use std::fmt;

use super::strings::indent_vec;
use super::{Format, OutputStyle};

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

/// A value that has been formatted with a [formatter][`Format`].
#[derive(Debug)]
pub struct FormattedOutput {
    segments: Vec<OutputSegment>,
}

impl FormattedOutput {
    /// Create a new [`FormattedOutput`] by formatting `value` with `format`.
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> crate::Result<Self>
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
        if spaces == 0 {
            return self;
        }

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

    /// Return a new [`FormattedOutput`] which has been indented by the given number of spaces.
    ///
    /// This is helpful when writing custom matchers that compose other matchers, so you can indent
    /// their output and include it in your matcher's output.
    pub fn indented(self, spaces: u32) -> Self {
        self.indented_inner(spaces, false)
    }

    /// Panic with this output as the error message.
    pub fn fail(&self) -> ! {
        panic!("\n{}\n", self);
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            f.write_fmt(format_args!("{}", segment.style.apply(&segment.buf)))?;
        }

        Ok(())
    }
}
