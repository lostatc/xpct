#![cfg(feature = "casefold")]

use std::borrow::Cow;

use unicase::UniCase;

use crate::core::Match;

use super::Mismatch;

/// The matcher for [`eq_casefold`].
///
/// [`eq_casefold`]: crate::eq_casefold
#[derive(Debug)]
pub struct EqCasefoldMatcher<'a> {
    expected: Cow<'a, str>,
}

impl<'a> EqCasefoldMatcher<'a> {
    /// Create a new [`EqCasefoldMatcher`] from the expected string.
    pub fn new(expected: impl Into<Cow<'a, str>>) -> Self {
        Self {
            expected: expected.into(),
        }
    }
}

impl<'a, Actual> Match<Actual> for EqCasefoldMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(UniCase::new(actual.as_ref()) == UniCase::new(self.expected.as_ref()))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}
