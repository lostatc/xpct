use crate::core::{DynMatchFailure, Format, Formatter, MatchFailure, Matcher};
use crate::matchers::NotMatcher;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct FailFormat;

impl FailFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for FailFormat {
    type Value = MatchFailure<DynMatchFailure>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        let fail = match value {
            MatchFailure::Pos(fail) => fail,
            MatchFailure::Neg(fail) => fail,
        };

        f.write_fmt(fail.into_fmt());

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn not<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
) -> Matcher<'a, In, NegOut, PosOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::new(NotMatcher::new(matcher), FailFormat::new())
}
