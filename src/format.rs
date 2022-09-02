use std::fmt;

use super::location::AssertionLocation;
use super::matcher::FailReason;
use super::indent::IndentWriter;

pub trait FormatError {
    fn fmt(&self, f: &mut ErrorFormatter) -> fmt::Result;
}

#[derive(Debug)]
pub struct ErrorFormatter {
    msg: IndentWriter<String>,
    reason: FailReason,
    name: Option<String>,
    location: Option<AssertionLocation>,
}

impl ErrorFormatter {
    pub(super) fn new(
        reason: FailReason,
        name: Option<String>,
        location: Option<AssertionLocation>,
    ) -> Self {
        Self {
            msg: IndentWriter::new(match &reason {
                FailReason::Fail(msg) => String::with_capacity(msg.len()),
                FailReason::Err(_) => String::new(),
            }),
            reason,
            name,
            location,
        }
    }

    pub fn as_str(&self) -> &str {
        self.msg.as_str()
    }

    pub fn reason(&self) -> &FailReason {
        &self.reason
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn location(&self) -> Option<&AssertionLocation> {
        self.location.as_ref()
    }

    pub fn indent(&self) -> u32 {
        self.msg.indent()
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.msg.set_indent(indent);
    }
}

impl fmt::Write for ErrorFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.msg.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.msg.write_char(c)
    }
}

impl AsRef<str> for ErrorFormatter {
    fn as_ref(&self) -> &str {
        self.msg.as_str()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultFormat;

impl FormatError for DefaultFormat {
    fn fmt(&self, _: &mut ErrorFormatter) -> fmt::Result {
        todo!();
    }
}
