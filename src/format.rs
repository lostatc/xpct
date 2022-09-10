use std::fmt;

use super::context::AssertionContext;
use super::result::{MatchError, MatchFailure};

pub trait ResultFormat: fmt::Display {
    type Pos;
    type Neg;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self;
}

pub trait AssertionFormat: fmt::Display {
    type Context;

    fn new(ctx: Self::Context, error: MatchError) -> Self;
}

#[derive(Debug)]
pub struct DefaultAssertionFormat {
    ctx: AssertionContext,
    error: MatchError,
}

impl fmt::Display for DefaultAssertionFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        /*
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
        */
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;

    fn new(ctx: Self::Context, error: MatchError) -> Self {
        Self { ctx, error }
    }
}
