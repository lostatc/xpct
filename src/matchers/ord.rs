use crate::core::SimpleMatch;

use super::Mismatch;

/// Which inequality test to perform with [`OrdMatcher`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Inequality {
    /// [`PartialOrd::lt`]
    Less,

    /// [`PartialOrd::le`]
    LessOrEqual,

    /// [`PartialOrd::gt`]
    Greater,

    /// [`PartialOrd::ge`]
    GreaterOrEqual,
}

/// The matcher for [`be_lt`], [`be_le`], [`be_gt`], and [`be_ge`].
///
/// [`be_lt`]: crate::be_lt
/// [`be_le`]: crate::be_le
/// [`be_gt`]: crate::be_gt
/// [`be_ge`]: crate::be_ge
#[derive(Debug)]
pub struct OrdMatcher<Expected> {
    expected: Expected,
    kind: Inequality,
}

impl<Expected> OrdMatcher<Expected> {
    /// Create a new [`OrdMatcher`].
    ///
    /// This accepts a `kind` which determines whether the behavior is `<`, `<=`, `>`, or `>=`.
    pub fn new(expected: Expected, kind: Inequality) -> Self {
        Self { expected, kind }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for OrdMatcher<Expected>
where
    Actual: PartialOrd<Expected>,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(match self.kind {
            Inequality::Less => actual < &self.expected,
            Inequality::LessOrEqual => actual <= &self.expected,
            Inequality::Greater => actual > &self.expected,
            Inequality::GreaterOrEqual => actual >= &self.expected,
        })
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}
