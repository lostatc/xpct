#![cfg(feature = "fmt")]

use std::convert::Infallible;

use super::{strings::indent, AssertionFormat, Format, Formatter};
use crate::core::{style, AssertionContext, AssertionFailure, MatchError};

#[derive(Debug, Default)]
pub struct DefaultAssertionFormat;

impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        f.write_char('\n');
        f.set_style(style::info());

        match (value.ctx.location, value.ctx.expr) {
            (Some(location), Some(expr)) => f.write_str(format!(
                "[{}:{}:{}] = {}\n",
                location.file, location.line, location.column, expr
            )),
            (Some(location), None) => f.write_str(format!(
                "{}:{}:{}\n",
                location.file, location.line, location.column
            )),
            (None, Some(expr)) => {
                f.write_str(expr);
                f.write_char('\n');
            }
            (None, None) => {}
        };

        f.reset_style();

        match value.error {
            MatchError::Fail(fail) => f.write_fmt(fail.into_fmt().indented(style::indent_len())),
            MatchError::Err(error) => {
                f.write_str(&indent(&error.to_string(), style::indent_len(), false))
            }
        }

        f.write_char('\n');

        Ok(())
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
}
