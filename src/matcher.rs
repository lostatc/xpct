use std::marker::PhantomData;

use super::format::ResultFormat;
use super::result::{MatchResult, MatchFailure};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchCase {
    /// We are expecting this matcher to match.
    Pos,

    /// We are expecting this matcher to not match.
    Neg,
}

impl MatchCase {
    pub fn is_pos(&self) -> bool {
        match self {
            Self::Pos => true,
            Self::Neg => false,
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::Pos => false,
            Self::Neg => true,
        }
    }
}

pub trait MatchBase {
    type In;
    type Success;
    type Fail;
}

pub trait Match: MatchBase {
    fn matches(&mut self, actual: &Self::In) -> anyhow::Result<MatchResult<Self::Success, Self::Fail>>;
}

pub trait MapPos: MatchBase {
    type PosOut;

    fn map_pos(&mut self, actual: Self::In)
        -> anyhow::Result<MatchResult<Self::PosOut, Self::Fail>>;
}

pub trait MapNeg: MatchBase {
    type NegOut;

    fn map_neg(&mut self, actual: Self::In)
        -> anyhow::Result<MatchResult<Self::NegOut, Self::Success>>;
}

pub trait DynMatchBase {
    type In;
}

pub trait DynMatch: DynMatchBase {
    fn matches(
        &mut self,
        case: MatchCase,
        actual: &Self::In,
    ) -> anyhow::Result<MatchResult<(), MatchFailure>>;
}

pub trait DynMapPos: DynMatchBase {
    type PosOut;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>>;
}

pub trait DynMapNeg: DynMatchBase {
    type NegOut;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>>;
}

pub trait DynMap: DynMapPos + DynMapNeg {}

pub struct Matcher<M, Fmt: ResultFormat> {
    matcher: M,
    result_fmt: PhantomData<Fmt>,
}

impl<M, Fmt: ResultFormat> Matcher<M, Fmt> {
    pub fn new(matcher: M) -> Self {
        Self {
            matcher,
            result_fmt: PhantomData,
        }
    }
}

impl<M, Fmt> DynMatchBase for Matcher<M, Fmt>
where
    M: MatchBase,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type In = M::In;
}

impl<M, Fmt> DynMatch for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    fn matches(
        &mut self,
        case: MatchCase,
        actual: &Self::In,
    ) -> anyhow::Result<MatchResult<(), MatchFailure>> {
        match self.matcher.matches(actual) {
            Ok(result) => match result {
                MatchResult::Success(success) => match case {
                    MatchCase::Pos => Ok(MatchResult::Success(())),
                    MatchCase::Neg => Ok(MatchResult::Fail(MatchFailure::success::<M::Success, Fmt>(success))),
                },
                MatchResult::Fail(fail) => match case {
                    MatchCase::Pos => Ok(MatchResult::Fail(MatchFailure::fail::<M::Fail, Fmt>(fail))),
                    MatchCase::Neg => Ok(MatchResult::Success(())),
                },
            },
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMapPos for Matcher<M, Fmt>
where
    M: MapPos,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type PosOut = M::PosOut;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>> {
        match self.matcher.map_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(
                MatchFailure::fail::<M::Fail, Fmt>(result),
            )),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMapNeg for Matcher<M, Fmt>
where
    M: MapNeg,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type NegOut = M::NegOut;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>> {
        match self.matcher.map_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(
                MatchFailure::success::<M::Success, Fmt>(result),
            )),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMap for Matcher<M, Fmt>
where
    M: MapPos + MapNeg,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
}

impl<M, Fmt> MatchBase for Matcher<M, Fmt>
where
    M: MatchBase,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type In = M::In;
    type Success = M::Success;
    type Fail = M::Fail;
}

impl<M, Fmt> MapPos for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type PosOut = M::In;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::Fail>> {
        match self.matcher.matches(&actual) {
            Ok(MatchResult::Success(_)) => Ok(MatchResult::Success(actual)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(result)),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> MapNeg for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    type NegOut = M::In;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::Success>> {
        match self.matcher.matches(&actual) {
            Ok(MatchResult::Success(result)) => Ok(MatchResult::Fail(result)),
            Ok(MatchResult::Fail(_)) => Ok(MatchResult::Success(actual)),
            Err(error) => Err(error),
        }
    }
}
