use crate::core::{Format, FormattedFailure, Formatter, MatchFailure, Matcher};
use crate::matchers::not::NotMatcher;

/// A formatter for [`FormattedFailure`] values.
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

    fn fmt(&self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.write_fmt(value.into_inner());
        Ok(())
    }
}

/// Negates the matcher passed to it.
///
/// This does the same thing as [`Assertion::to_not`].
///
/// # Examples
///
/// ```
/// use xpct::{expect, not, equal};
///
/// expect!("foo").to(not(equal("bar")));
/// ```
///
/// [`Assertion::to_not`]: crate::core::Assertion::to_not
pub fn not<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
) -> Matcher<'a, In, NegOut, PosOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::transform(NotMatcher::new(matcher), FailureFormat::new())
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
