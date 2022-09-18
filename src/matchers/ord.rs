use crate::core::SimpleMatch;

use super::Mismatch;

/// Which inequality test to perform.
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

#[derive(Debug)]
pub struct OrdMatcher<Expected> {
    expected: Expected,
    kind: Inequality,
}

impl<Expected> OrdMatcher<Expected> {
    pub fn new(expected: Expected, kind: Inequality) -> Self {
        Self { expected, kind }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for OrdMatcher<Expected>
where
    Actual: PartialOrd<Expected>,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> anyhow::Result<bool> {
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
