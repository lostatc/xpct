#![cfg(not(feature = "color"))]

// Nothing in this module should have public-facing API documentation. Since the API is identical,
// we elect to only show the versions of these types with `#[cfg(feature = "color")]` in the API
// docs, so all the doc comments should go there.

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

    fn indented_inner(
        &mut self,
        prefix: impl AsRef<str>,
        hanging: bool,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let mut formatter = Self::new();
        func(&mut formatter)?;

        let output = FormattedOutput { buf: formatter.buf };

        let indented = output.indented_inner(prefix.as_ref(), hanging);
        self.buf.push_str(&indented.buf);

        Ok(())
    }

    pub fn indented(
        &mut self,
        prefix: impl AsRef<str>,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        self.indented_inner(prefix, false, func)
    }

    pub fn indented_hanging(
        &mut self,
        prefix: impl AsRef<str>,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        self.indented_inner(prefix, true, func)
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

    fn indented_inner(mut self, prefix: impl AsRef<str>, hanging: bool) -> Self {
        if !prefix.as_ref().is_empty() {
            self.buf = strings::indent(&self.buf, prefix.as_ref(), hanging).into();
        }

        self
    }

    pub fn indented(self, prefix: impl AsRef<str>) -> Self {
        self.indented_inner(prefix, false)
    }

    pub fn indented_hanging(self, prefix: impl AsRef<str>) -> Self {
        self.indented_inner(prefix, true)
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
