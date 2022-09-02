use std::borrow::Cow;

use crate::location::AssertionLocation;
use crate::matcher::FailReason;

const INDENT_PREFIX: &'static str = "    ";

fn indent<'a>(prefix: impl Into<Cow<'a, str>>, text: impl Into<Cow<'a, str>>) -> String {
    let prefix = prefix.into();
    let text = text.into();
    let length = text.len();
    let mut output = String::with_capacity(length + length / 2);

    for (i, line) in text.lines().enumerate() {
        if i > 0 {
            output.push('\n');
        }

        if !line.is_empty() {
            output.push_str(&prefix);
        }

        output.push_str(line);
    }

    if text.ends_with('\n') {
        output.push('\n');
    }

    output
}

pub trait FormatError {
    fn fmt(&self, f: &mut ErrorFormatter);
}

#[derive(Debug)]
pub struct ErrorFormatter {
    buf: String,
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
            buf: match &reason {
                FailReason::Fail(msg) => String::with_capacity(msg.len()),
                FailReason::Err(_) => String::new(),
            },
            reason,
            name,
            location,
        }
    }
}

impl ErrorFormatter {
    pub fn reason(&self) -> &FailReason {
        &self.reason
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn location(&self) -> Option<&AssertionLocation> {
        self.location.as_ref()
    }

    pub fn msg(&mut self) -> &mut String {
        &mut self.buf
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultFormat;

impl FormatError for DefaultFormat {
    fn fmt(&self, _: &mut ErrorFormatter) {
        todo!();
    }
}
