#![cfg(not(feature = "color"))]

use std::fmt;

use super::color::OutputStyle;
use super::{strings, Format};

#[derive(Debug)]
pub struct Formatter {
    buf: String,
    style: OutputStyle,
}

impl Formatter {
    fn new() -> Self {
        Self {
            buf: String::new(),
            style: Default::default(),
        }
    }

    pub fn write_str(&mut self, s: impl AsRef<str>) {
        self.buf.push_str(s.as_ref());
    }

    pub fn write_char(&mut self, c: char) {
        self.buf.push(c);
    }

    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        self.buf.push_str(&output.into().buf);
    }

    pub fn style(&self) -> &OutputStyle {
        &self.style
    }

    pub fn set_style(&mut self, style: OutputStyle) {
        self.style = style;
    }

    pub fn reset_style(&mut self) {
        self.style = Default::default();
    }
}

#[derive(Debug)]
pub struct FormattedOutput {
    buf: String,
}

impl FormattedOutput {
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> crate::Result<Self>
    where
        Fmt: Format<Value = Value>,
    {
        let mut formatter = Formatter::new();
        format.fmt(&mut formatter, value)?;
        Ok(Self { buf: formatter.buf })
    }

    pub fn indented(mut self, spaces: u32) -> Self {
        if spaces > 0 {
            self.buf = strings::indent(&self.buf, spaces, false).into();
        }

        self
    }

    pub fn fail(&self) -> ! {
        panic!("\n{}\n", self);
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.buf)
    }
}
