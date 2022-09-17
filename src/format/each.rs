use std::convert::Infallible;

use crate::core::{strings, style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher};
use crate::matchers::{EachContext, EachMatcher, SomeFailures};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SomeFailuresFormat;

impl SomeFailuresFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
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
                    f.write_str(style::FAILED_MSG);
                    f.reset_style();
                    f.write_char('\n');

                    f.write_fmt(fail.into_fmt().indented(failure_indent));
                }
                None => {
                    f.set_style(style::success());
                    f.write_str(style::MATCHED_MSG);
                    f.reset_style();
                    f.write_char('\n');
                }
            }

            f.write_char('\n');
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct EachFormat<Fmt> {
    inner: Fmt,
}

impl<Fmt> EachFormat<Fmt> {
    pub fn new(inner: Fmt) -> Self {
        Self { inner }
    }
}

impl<Fmt> Format for EachFormat<Fmt>
where
    Fmt: Format<Value = SomeFailures>,
{
    type Value = MatchFailure<SomeFailures, Infallible>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::important());
        f.write_str("Expected all of these to match:\n");
        f.reset_style();

        match value {
            MatchFailure::Pos(fail) => {
                f.write_fmt(FormattedOutput::new(fail, self.inner)?.indented(style::indent_len(1)))
            }
            MatchFailure::Neg(_) => unreachable!(),
        };

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn each<'a, T>(block: impl FnOnce(&mut EachContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(
        EachMatcher::new(block),
        EachFormat::<SomeFailuresFormat>::default(),
    )
}
