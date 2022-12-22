use crate::core::{Format, FormattedFailure, Formatter, MatchFailure, Matcher};
use crate::matchers::NotMatcher;

/// A formatter for a pre-formatted [`FormattedFailure`] value.
///
/// This formatter just writes a pre-formatted value via [`Formatter::write_fmt`]. It's mostly
/// useful for combinator matchers which need to print the output of the matchers they compose.
///
/// If you need to print multiple [`FormattedFailure`] values, use [`SomeFailuresFormat`].
///
/// [`SomeFailuresFormat`]: crate::format::SomeFailuresFormat
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct FailureFormat;

impl FailureFormat {
    /// Create a new [`FailureFormat`].
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

/// Negates the matcher passed to it.
///
/// This does the same thing as [`Assertion::to_not`].
///
/// [`Assertion::to_not`]: crate::core::Assertion::to_not
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

#[cfg(test)]
mod tests {
    use super::not;
    use crate::{be_true, expect};

    #[test]
    fn succeeds_when_matcher_ok() {
        expect!(false).to(not(be_true()));
    }

    #[test]
    fn succeeds_when_matcher_fails() {
        expect!(true).to_not(not(be_true()));
    }

    #[test]
    #[should_panic]
    fn fails_when_matcher_ok() {
        expect!(false).to_not(not(be_true()));
    }

    #[test]
    #[should_panic]
    fn fails_when_matcher_fails() {
        expect!(true).to(not(be_true()));
    }
}
