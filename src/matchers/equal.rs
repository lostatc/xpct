use crate::{MatchFailure, Format, Formatter, ResultFormat, Matcher, SimpleMatch};

#[derive(Debug)]
pub struct Mismatch<Actual, Expected> {
    pub actual: Actual,
    pub expected: Expected,
}

#[derive(Debug)]
pub struct MismatchFormat<Actual, Expected>(MatchFailure<Mismatch<Actual, Expected>>);

impl<Actual, Expected> Format for MismatchFormat<Actual, Expected> {
    fn fmt(&self, _: &mut Formatter) {
        todo!()
    }
}

impl<Actual, Expected> ResultFormat for MismatchFormat<Actual, Expected> {
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

pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: PartialEq<Expected> + Eq + 'a,
    Expected: 'a,
{
    Matcher::simple::<MismatchFormat<Actual, Expected>, _>(EqualMatcher::new(expected))
}
