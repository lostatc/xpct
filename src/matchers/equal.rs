use crate::{MatchFailure, Format, Formatter, ResultFormat, Matcher, SimpleMatch};

#[derive(Debug)]
pub struct Mismatch<Actual, Expected> {
    pub actual: Actual,
    pub expected: Expected,
}

#[derive(Debug)]
pub struct MismatchFormat<Actual, Expected>(MatchFailure<Mismatch<Actual, Expected>>);

impl<Actual, Expected> Format for MismatchFormat<Actual, Expected> {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl<Actual, Expected> ResultFormat for MismatchFormat<Actual, Expected>
where
    Actual: 'static,
    Expected: 'static,
{
    type Pos = Mismatch<Actual, Expected>;
    type Neg = Mismatch<Actual, Expected>;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub struct EqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqualMatcher<Expected> {
    pub fn new(expected: Expected) -> Self {
        Self {
            expected,
        }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for EqualMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type Fail = Mismatch<Actual, Expected>;

    fn matches(&mut self, actual: &Actual) -> anyhow::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}

pub fn equal<Actual, Expected>(expected: Expected) -> Matcher<Actual, Actual>
where
    Actual: PartialEq<Expected> + Eq + 'static,
    Expected: 'static,
{
    Matcher::simple::<MismatchFormat<Actual, Expected>, _>(EqualMatcher::new(expected))
}
