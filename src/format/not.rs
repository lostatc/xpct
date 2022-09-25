use crate::core::{Format, FormattedFailure, Formatter, MatchFailure, Matcher};
use crate::matchers::NotMatcher;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct FailureFormat;

impl FailureFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for FailureFormat {
    type Value = MatchFailure<FormattedFailure>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        let fail = match value {
            MatchFailure::Pos(fail) => fail,
            MatchFailure::Neg(fail) => fail,
        };

        f.write_fmt(fail);

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
    Matcher::new(NotMatcher::new(matcher), FailureFormat::new())
}
