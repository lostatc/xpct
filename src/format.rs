use std::fmt;

use super::error::DynMatchError;
use super::indent::IndentWriter;
use super::context::AssertionContext;

pub trait Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result;
}

#[derive(Debug)]
pub struct Formatter {
    msg: IndentWriter<String>,
}

impl Formatter {
    pub(super) fn new() -> Self {
        Self {
            msg: IndentWriter::new(String::new()),
        }
    }

    pub fn as_str(&self) -> &str {
        self.msg.as_str()
    }

    pub fn indent(&self) -> u32 {
        self.msg.indent()
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.msg.set_indent(indent);
    }
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.msg.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.msg.write_char(c)
    }
}

impl AsRef<str> for Formatter {
    fn as_ref(&self) -> &str {
        self.msg.as_str()
    }
}

pub struct DefaultErrorFormat(anyhow::Error);

impl Display for DefaultErrorFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl From<anyhow::Error> for DefaultErrorFormat {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

pub trait AssertionFormat: Display {
    type Context;
    
    fn new(ctx: Self::Context, error: DynMatchError) -> Self;
}

pub struct DefaultAssertionFormat {
    ctx: AssertionContext,
    error: DynMatchError,
}

impl Display for DefaultAssertionFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
    
    fn new(ctx: Self::Context, error: DynMatchError) -> Self {
        Self { ctx, error }
    }
}
