use crate::core::{Format, FormattedFailure, Formatter, MatchFailure, Matcher};
use crate::matchers::NotMatcher;

/// A formatter for a pre-formatted [`FormattedFailure`] value.
///
/// This formatter just writes a pre-formatted value via [`Formatter::write_fmt`]. It's mostly
/// useful for combinator matchers which need to print the output of the matchers they compose.
///
/// If you need to print multiple [`FormattedFailure`] values, use [`SomeFailuresFormat`].
///
/// # Examples
///
/// ```
/// use xpct::core::Matcher;
/// use xpct::matchers::NotMatcher;
/// use xpct::format::FailureFormat;
///
/// # use xpct::matchers::BeTrueMatcher;
/// # use xpct::format::MessageFormat;
/// # let inner_matcher = Matcher::simple(BeTrueMatcher::new(), MessageFormat::new("", ""));
/// let matcher = Matcher::new(NotMatcher::new(inner_matcher), FailureFormat::new());
/// ```
///
/// [`SomeFailuresFormat`]: crate::format::SomeFailuresFormat
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
