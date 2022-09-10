use std::fmt;

use crate::{MatchFailure, Matcher, ResultFormat, SimpleMatch};

#[derive(Debug)]
pub struct Mismatch<Actual, Expected> {
    pub actual: Actual,
    pub expected: Expected,
}

#[derive(Debug)]
pub struct EqualFormat<Actual, Expected>(MatchFailure<Mismatch<Actual, Expected>>);

impl<Actual, Expected> fmt::Display for EqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        /*
        fn fmt(&self, f: &mut Formatter) {
            match &self.0 {
                MatchFailure::Pos(mismatch) => labeled_mismatch(
                    f,
                    LabeledMismatch {
                        expected_label: "Expected:",
                        actual_label: "To equal:",
                        mismatch,
                    },
                ),
                MatchFailure::Neg(mismatch) => labeled_mismatch(
                    f,
                    LabeledMismatch {
                        expected_label: "Expected:",
                        actual_label: "To not equal:",
                        mismatch,
                    },
                ),
            }
        }
        */
    }
}

impl<Actual, Expected> ResultFormat for EqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
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
        Self { expected }
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
    Actual: PartialEq<Expected> + Eq + fmt::Debug + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple::<EqualFormat<Actual, Expected>, _>(EqualMatcher::new(expected))
}
