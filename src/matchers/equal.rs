use crate::core::SimpleMatch;

#[derive(Debug)]
pub struct Mismatch<Actual, Expected> {
    pub actual: Actual,
    pub expected: Expected,
}

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

    fn matches(&mut self, actual: &Actual) -> anyhow::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual: self.expected,
            expected: actual,
        }
    }
}

#[cfg(feature = "fmt")]
use {crate::core::Matcher, std::fmt};

#[cfg(feature = "fmt")]
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    use super::format::EqualFormat;

    Matcher::simple(EqualMatcher::new(expected), EqualFormat::new())
}
