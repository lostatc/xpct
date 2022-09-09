use std::marker::PhantomData;

use crate::{MatchFailure, Format, Formatter, ResultFormat, MatchBase, MatchPos, MatchResult, MatchNeg, Matcher};

#[derive(Debug)]
pub struct Mismatch<Lhs, Rhs> {
    pub expected: Rhs,
    pub actual: Lhs,
}

#[derive(Debug)]
pub struct EqualFormat<Lhs, Rhs>(MatchFailure<Mismatch<Lhs, Rhs>, Mismatch<Lhs, Rhs>>);

impl<Lhs, Rhs> Format for EqualFormat<Lhs, Rhs> {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl<Lhs, Rhs> ResultFormat for EqualFormat<Lhs, Rhs>
where
    Lhs: 'static,
    Rhs: 'static,
{
    type Pos = Mismatch<Lhs, Rhs>;
    type Neg = Mismatch<Lhs, Rhs>;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub struct EqualMatcher<Lhs, Rhs> {
    expected: Rhs,
    actual: PhantomData<Lhs>,
}

impl<Lhs, Rhs> EqualMatcher<Lhs, Rhs> {
    pub fn new(expected: Rhs) -> Self {
        Self {
            expected,
            actual: PhantomData,
        }
    }
}

impl<Lhs, Rhs> MatchBase for EqualMatcher<Lhs, Rhs> {
    type In = Lhs;
}

impl<Lhs, Rhs> MatchPos for EqualMatcher<Lhs, Rhs>
where
    Lhs: PartialEq<Rhs> + Eq,
{
    type PosOut = Lhs;
    type PosFail = Mismatch<Lhs, Rhs>;

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

impl<Lhs, Rhs> MatchNeg for EqualMatcher<Lhs, Rhs>
where
    Lhs: PartialEq<Rhs> + Eq,
{
    type NegOut = Lhs;
    type NegFail = Mismatch<Lhs, Rhs>;

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

pub fn equal<Lhs, Rhs>(actual: Rhs) -> Matcher<Lhs, Lhs, Lhs>
where
    Lhs: PartialEq<Rhs> + Eq + 'static,
    Rhs: 'static,
{
    Matcher::new::<_, EqualFormat<Lhs, Rhs>>(EqualMatcher::new(actual))
}
