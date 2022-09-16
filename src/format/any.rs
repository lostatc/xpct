use std::convert::Infallible;

use crate::core::{
    strings, style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher, ResultFormat,
};
use crate::matchers::{AllFailures, AnyContext, AnyMatcher};

pub struct AllFailuresFormat;

impl Format for AllFailuresFormat {
    type Value = AllFailures;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        let num_failures = value.len();
        let failure_indent = strings::int_len(num_failures, 10) + 4;

        for (i, fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}[{}]  ",
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            f.set_style(style::failure());
            f.write_str("FAILED");
            f.reset_style();
            f.write_char('\n');

            f.write_fmt(fail.into_fmt().indented(failure_indent));
            f.write_char('\n');
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AnyFormat;

impl Format for AnyFormat {
    type Value = MatchFailure<AllFailures, Infallible>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        f.set_style(style::important());
        f.write_str("Expected at least one of these to match:\n");
        f.reset_style();

        match value {
            MatchFailure::Pos(fail) => f.write_fmt(
                FormattedOutput::new(fail, AllFailuresFormat)?.indented(style::indent_len()),
            ),
            MatchFailure::Neg(_) => unreachable!(),
        };

        Ok(())
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = Infallible;
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn any<'a, T>(block: impl Fn(&mut AnyContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(AnyMatcher::new(block), AnyFormat)
}
