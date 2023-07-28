#![cfg(feature = "casefold")]

use std::{borrow::Cow, fmt};

use crate::core::Matcher;
use crate::matchers::EqCasefoldMatcher;

use super::MismatchFormat;

/// Succeeds when the actual string equals the expected string regardless of case.
///
/// This uses Unicode case folding as opposed to just [`str::to_lowercase`].
///
/// # Examples
///
/// ```
/// use xpct::{expect, eq_casefold};
///
/// expect!("Fun").to(eq_casefold("fun"));
/// expect!("Spaß").to(eq_casefold("Spass"));
/// ```
pub fn eq_casefold<'a, Actual>(expected: impl Into<Cow<'a, str>>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::new(
        EqCasefoldMatcher::new(expected),
        MismatchFormat::new(
            "to equal case-insensitively",
            "to not equal case-insensitively",
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::eq_casefold;
    use crate::expect;

    #[test]
    fn succeeds_when_equal() {
        expect!("Spaß").to(eq_casefold("spass"));
    }

    #[test]
    fn succeeds_when_not_equal() {
        expect!("Spaß").to_not(eq_casefold("spas"));
    }

    #[test]
    #[should_panic]
    fn fails_when_equal() {
        expect!("Spaß").to_not(eq_casefold("spass"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_equal() {
        expect!("Spaß").to(eq_casefold("spas"));
    }
}
