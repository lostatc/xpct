use std::convert::Infallible;

use crate::core::{
    strings, style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher, ResultFormat,
};
use crate::matchers::{EachContext, EachMatcher, SomeFailures};

pub struct SomeFailuresFormat;

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        let num_failures = value.len();
        let failure_indent = strings::int_len(num_failures, 10) + 4;

        for (i, maybe_fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}[{}]  ",
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            match maybe_fail {
                Some(fail) => {
                    f.set_style(style::failure());
                    f.write_str("FAILED");
                    f.reset_style();
                    f.write_char('\n');

                    f.write_fmt(fail.into_fmt().indented(failure_indent));
                }
                None => {
                    f.set_style(style::success());
                    f.write_str("MATCHED");
                    f.reset_style();
                    f.write_char('\n');
                }
            }

            f.write_char('\n');
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct EachFormat;

impl Format for EachFormat {
    type Value = MatchFailure<SomeFailures, Infallible>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        f.set_style(style::important());
        f.write_str("Expected all of these to match:\n");
        f.reset_style();

        match value {
            MatchFailure::Pos(fail) => f.write_fmt(
                FormattedOutput::new(fail, SomeFailuresFormat)?.indented(style::indent_len()),
            ),
            MatchFailure::Neg(_) => unreachable!(),
        };

        Ok(())
    }
}

impl ResultFormat for EachFormat {
    type Pos = SomeFailures;
    type Neg = Infallible;
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn each<'a, T>(block: impl FnOnce(&mut EachContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(EachMatcher::new(block), EachFormat)
}
