use std::borrow::Cow;

use crate::core::SimpleMatch;

use super::Mismatch;

/// The matcher for [`contain_substr`].
///
/// [`contain_substr`]: crate::contain_substr
#[derive(Debug)]
pub struct ContainSubstrMatcher<'a> {
    substr: Cow<'a, str>,
}

impl<'a> ContainSubstrMatcher<'a> {
    /// Create a new [`ContainSubstrMatcher`] from the expected substring.
    pub fn new(substr: impl Into<Cow<'a, str>>) -> Self {
        Self {
            substr: substr.into(),
        }
    }
}

impl<'a, Actual> SimpleMatch<Actual> for ContainSubstrMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().contains(self.substr.as_ref()))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.substr,
            actual,
        }
    }
}
