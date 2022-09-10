use std::fmt;

use super::context::AssertionContext;
use super::indent::IndentWriter;
use super::result::{MatchError, MatchFailure};

pub trait Format {
    fn fmt(&self, f: &mut Formatter);
}

impl fmt::Display for dyn Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatter = Formatter::new();
        self.fmt(&mut formatter);
        f.write_str(formatter.as_str())
    }
}

#[derive(Debug)]
pub struct Formatter {
    msg: IndentWriter,
}

impl Formatter {
    pub(super) fn new() -> Self {
        Self {
            msg: IndentWriter::new(String::new()),
        }
    }

    pub(super) fn into_inner(self) -> String {
        self.msg.into_inner()
    }

    pub(super) fn as_str(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn indent(&self) -> u32 {
        self.msg.indent()
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.msg.set_indent(indent);
    }

    pub fn write_str(&mut self, s: &str) {
        self.msg.write_str(s);
    }

    pub fn write_char(&mut self, c: char) {
        self.msg.write_char(c);
    }
}

pub trait ResultFormat: Format {
    type Pos;
    type Neg;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self;
}

pub trait AssertionFormat: Format {
    type Context;

    fn new(ctx: Self::Context, error: MatchError) -> Self;
}

pub struct DefaultAssertionFormat {
    ctx: AssertionContext,
    error: MatchError,
}

impl Format for DefaultAssertionFormat {
    fn fmt(&self, _: &mut Formatter) {
        todo!()
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;

    fn new(ctx: Self::Context, error: MatchError) -> Self {
        Self { ctx, error }
    }
}
