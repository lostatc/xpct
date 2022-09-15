use std::convert::Infallible;

use crate::core::{strings, style, Format, Formatter, MatchFailure, Matcher, ResultFormat};
use crate::matchers::{AllFailures, AnyContext, AnyMatcher, SomeFailures};

pub struct AllFailuresFormat;

impl Format for AllFailuresFormat {
    type Value = AllFailures;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        f.set_style(style::important());
        f.write_str("Expected none of these to fail:\n");
        f.reset_style();

        let num_failures = value.len();
        let failure_indent = style::indent_len() + strings::int_len(num_failures, 10) + 4;

        for (i, fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}{}[{}]  ",
                style::indent(),
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

pub struct SomeFailuresFormat;

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        f.set_style(style::important());
        f.write_str("Expected all of these to fail:\n");
        f.reset_style();

        let num_failures = value.len();
        let failure_indent = style::indent_len() + strings::int_len(num_failures, 10) + 4;

        for (i, maybe_fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}{}[{}]  ",
                style::indent(),
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            match maybe_fail {
                Some(fail) => {
                    f.set_style(style::success());
                    f.write_str("FAILED");
                    f.reset_style();
                    f.write_char('\n');

                    f.write_fmt(fail.into_fmt().indented(failure_indent));
                }
                None => {
                    f.set_style(style::failure());
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
pub struct AnyFormat;

impl Format for AnyFormat {
    type Value = MatchFailure<AllFailures, SomeFailures>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        match value {
            MatchFailure::Pos(fail) => AllFailuresFormat.fmt(f, fail),
            MatchFailure::Neg(fail) => SomeFailuresFormat.fmt(f, fail),
        }
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;
}

pub fn any<'a, T>(block: impl Fn(&mut AnyContext<T>) + 'a) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::new(AnyMatcher::new(block), AnyFormat)
}
