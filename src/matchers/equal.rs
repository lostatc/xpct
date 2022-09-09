use std::marker::PhantomData;

use crate::{MatchFailure, Format, Formatter, ResultFormat, MatchBase, MatchPos, MatchResult, MatchNeg, Matcher};

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

pub struct EqualMatcher<Actual, Expected> {
    expected: Expected,
    actual: PhantomData<Actual>,
}

impl<Actual, Expected> EqualMatcher<Actual, Expected> {
    pub fn new(expected: Expected) -> Self {
        Self {
            expected,
            actual: PhantomData,
        }
    }
}

impl<Actual, Expected> MatchBase for EqualMatcher<Actual, Expected> {
    type In = Actual;
}

impl<Actual, Expected> MatchPos for EqualMatcher<Actual, Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type PosOut = Actual;
    type PosFail = Mismatch<Actual, Expected>;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        if actual == self.expected {
            Ok(MatchResult::Success(actual))
        } else {
            Ok(MatchResult::Fail(Mismatch {
                actual,
                expected: self.expected,
            }))
        }
    }
}

impl<Actual, Expected> MatchNeg for EqualMatcher<Actual, Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type NegOut = Actual;
    type NegFail = Mismatch<Actual, Expected>;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        if actual == self.expected {
            Ok(MatchResult::Fail(Mismatch {
                actual,
                expected: self.expected,
            }))
        } else {
            Ok(MatchResult::Success(actual))
        }
    }
}

pub fn equal<Actual, Expected>(expected: Expected) -> Matcher<Actual, Actual>
where
    Actual: PartialEq<Expected> + Eq + 'static,
    Expected: 'static,
{
    Matcher::new::<_, MismatchFormat<Actual, Expected>>(EqualMatcher::new(expected))
}
