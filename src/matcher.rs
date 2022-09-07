use std::marker::PhantomData;

use super::format::ResultFormat;
use super::result::{MatchFailure, MatchResult, Matches};

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
    type Res: Matches;
}

pub trait Match: MatchBase {
    fn matches(&mut self, actual: &Self::In) -> anyhow::Result<Self::Res>;
}

pub trait MapPos: MatchBase {
    type PosOut;

    fn map_pos(&mut self, actual: Self::In)
        -> anyhow::Result<MatchResult<Self::PosOut, Self::Res>>;
}

pub trait MapNeg: MatchBase {
    type NegOut;

    fn map_neg(&mut self, actual: Self::In)
        -> anyhow::Result<MatchResult<Self::NegOut, Self::Res>>;
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
    Fmt: ResultFormat<Res = M::Res>,
{
    type In = M::In;
}

impl<M, Fmt> DynMatch for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Res = M::Res>,
{
    fn matches(
        &mut self,
        case: MatchCase,
        actual: &Self::In,
    ) -> anyhow::Result<MatchResult<(), MatchFailure>> {
        match (self.matcher.matches(actual), case) {
            (Ok(result), MatchCase::Pos) if result.matches() => Ok(MatchResult::Success(())),
            (Ok(result), MatchCase::Neg) if !result.matches() => Ok(MatchResult::Success(())),
            (Ok(result), _) => Ok(MatchResult::Fail(MatchFailure::new::<M::Res, Fmt>(
                result, case,
            ))),
            (Err(error), _) => Err(error),
        }
    }
}

impl<M, Fmt> DynMapPos for Matcher<M, Fmt>
where
    M: MapPos,
    Fmt: ResultFormat<Res = M::Res>,
{
    type PosOut = M::PosOut;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>> {
        match self.matcher.map_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(
                MatchFailure::new::<M::Res, Fmt>(result, MatchCase::Pos),
            )),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMapNeg for Matcher<M, Fmt>
where
    M: MapNeg,
    Fmt: ResultFormat<Res = M::Res>,
{
    type NegOut = M::NegOut;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>> {
        match self.matcher.map_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(
                MatchFailure::new::<M::Res, Fmt>(result, MatchCase::Neg),
            )),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMap for Matcher<M, Fmt>
where
    M: MapPos + MapNeg,
    Fmt: ResultFormat<Res = M::Res>,
{
}

impl<M, Fmt> MatchBase for Matcher<M, Fmt>
where
    M: MatchBase,
    Fmt: ResultFormat<Res = M::Res>,
{
    type In = M::In;
    type Res = M::Res;
}

impl<M, Fmt> MapPos for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Res = M::Res>,
{
    type PosOut = M::In;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::Res>> {
        match self.matcher.matches(&actual) {
            Ok(result) if result.matches() => Ok(MatchResult::Success(actual)),
            Ok(result) => Ok(MatchResult::Fail(result)),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> MapNeg for Matcher<M, Fmt>
where
    M: Match,
    Fmt: ResultFormat<Res = M::Res>,
{
    type NegOut = M::In;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::Res>> {
        match self.matcher.matches(&actual) {
            Ok(result) if !result.matches() => Ok(MatchResult::Success(actual)),
            Ok(result) => Ok(MatchResult::Fail(result)),
            Err(error) => Err(error),
        }
    }
}
