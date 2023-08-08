use crate::core::Match;

use crate::matchers::Mismatch;

/// The matcher for [`be_default`].
///
/// [`be_default`]: crate::be_default
#[derive(Debug, Default)]
pub struct BeDefaultMatcher<Actual> {
    expected: Actual,
}

impl<Actual> BeDefaultMatcher<Actual>
where
    Actual: Default,
{
    /// Create a new [`BeDefaultMatcher`] from the expected value.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Actual> Match<Actual> for BeDefaultMatcher<Actual>
where
    Actual: Default + PartialEq<Actual> + Eq,
{
    type Fail = Mismatch<Actual, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}
