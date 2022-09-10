use super::context::AssertionContext;
use super::indent::IndentWriter;
use super::result::{MatchError, MatchFailure};

pub trait Format {
    fn fmt(&self, f: &mut Formatter);
}

#[derive(Debug)]
pub struct Formatter {
    msg: IndentWriter,
}

pub fn format(fmt: &impl Format) -> String {
    let mut formatter = Formatter::new();
    fmt.fmt(&mut formatter);
    formatter.into_inner()
}

impl Formatter {
    pub(super) fn new() -> Self {
        Self {
            msg: IndentWriter::new(),
        }
    }

    pub(super) fn into_inner(self) -> String {
        self.msg.into_inner()
    }

    pub(super) fn as_str(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn indent(&mut self) -> u32 {
        self.msg.indent()
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.msg.set_indent(indent);
    }

    pub fn write_fmt(&mut self, fmt: &impl Format) {
        let mut formatter = Self::new();
        fmt.fmt(&mut formatter);
        self.write_str(formatter.as_str().trim());
    }

    pub fn write_str(&mut self, s: impl AsRef<str>) {
        self.msg.write_str(s.as_ref());
    }

    pub fn write_char(&mut self, c: char) {
        self.msg.write_char(c);
    }

    pub fn writeln(&mut self) {
        self.msg.write_char('\n');
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

#[derive(Debug)]
pub struct DefaultAssertionFormat {
    ctx: AssertionContext,
    error: MatchError,
}

impl Format for DefaultAssertionFormat {
    fn fmt(&self, f: &mut Formatter) {
        match (&self.ctx.location, &self.ctx.expr) {
            (Some(location), Some(expr)) => {
                f.write_str(&format!(
                    "[{}:{}:{}] {}",
                    location.file, location.line, location.column, expr
                ));
            }
            (Some(location), None) => {
                f.write_str(&format!(
                    "[{}:{}:{}]",
                    location.file, location.line, location.column
                ));
            }
            (None, Some(expr)) => {
                f.write_str(&format!("{}", expr));
            }
            _ => {}
        }

        f.writeln();
        f.set_indent(2);
        f.write_str(&self.error.to_string());
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;

    fn new(ctx: Self::Context, error: MatchError) -> Self {
        Self { ctx, error }
    }
}
