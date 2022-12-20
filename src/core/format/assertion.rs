use super::{strings, Format, FormattedOutput, Formatter};
use crate::core::{style, AssertionContext, AssertionFailure, MatchError};

/// The provided implementation of [`AssertionFormat`].
///
/// You shouldn't ever need to implement this trait yourself, but you can if you want to customize
/// the formatting of failed assertions. However, instead of using the [`expect!`] macro, you will
/// need to use the [`expect`] function which allows you to specify a custom formatter.
///
/// This [`AssertionFormat`] implementation prints the expression that was passed to [`expect!`]
/// along with the file name, line number, and column number.
///
/// [`expect!`]: crate::expect!
/// [`expect`]: crate::core::expect
/// [`Assertionformat`]: crate::core::AssertionFormat
#[derive(Debug, Default)]
pub struct DefaultAssertionFormat;

impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
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
            MatchError::Fail(fail) => {
                f.write_fmt(FormattedOutput::from(fail).indented(style::indent_len(1)))
            }
            MatchError::Err(error) => f.write_str(&strings::indent(
                &error.to_string(),
                style::indent_len(1),
                false,
            )),
        }

        f.write_char('\n');

        Ok(())
    }
}
