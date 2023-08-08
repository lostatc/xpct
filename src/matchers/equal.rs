use crate::core::Match;

use super::Mismatch;

/// The matcher for [`equal`].
///
/// [`equal`]: crate::equal
#[derive(Debug)]
pub struct EqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqualMatcher<Expected> {
    /// Create a new [`EqualMatcher`] from the expected value.
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> Match<Actual> for EqualMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}
