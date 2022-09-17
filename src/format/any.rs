use std::convert::Infallible;

use crate::core::{strings, style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher};
use crate::matchers::{AllFailures, AnyContext, AnyMatcher};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct AllFailuresFormat;

impl AllFailuresFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for AllFailuresFormat {
    type Value = AllFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
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
            f.write_str(style::FAILED_MSG);
            f.reset_style();
            f.write_char('\n');

            f.write_fmt(fail.into_fmt().indented(failure_indent));
            f.write_char('\n');
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct AnyFormat<Fmt> {
    inner: Fmt,
}

impl<Fmt> AnyFormat<Fmt> {
    pub fn new(inner: Fmt) -> Self {
        Self { inner }
    }
}

impl<Fmt> Format for AnyFormat<Fmt>
where
    Fmt: Format<Value = AllFailures>,
{
    type Value = MatchFailure<AllFailures, Infallible>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::important());
        f.write_str("Expected at least one of these to match:\n");
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
pub fn any<'a, T>(block: impl Fn(&mut AnyContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(
        AnyMatcher::new(block),
        AnyFormat::<AllFailuresFormat>::default(),
    )
}
