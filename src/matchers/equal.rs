use crate::core::SimpleMatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch<Expected, Actual> {
    pub expected: Expected,
    pub actual: Actual,
}

#[derive(Debug)]
pub struct EqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqualMatcher<Expected> {
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for EqualMatcher<Expected>
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
