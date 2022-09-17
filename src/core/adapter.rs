use std::{borrow::Borrow, marker::PhantomData};

use super::{
    DynMatchFailure, DynMatchNeg, DynMatchPos, MatchBase, MatchFailure, MatchNeg, MatchPos,
    MatchResult, ResultFormat, SimpleMatch,
};

#[derive(Debug)]
pub(super) struct DynMatchAdapter<M, Fmt: ResultFormat> {
    matcher: M,
    format: Fmt,
}

impl<M, Fmt: ResultFormat> DynMatchAdapter<M, Fmt> {
    pub fn new(matcher: M, format: Fmt) -> Self {
        Self { matcher, format }
    }
}

impl<M, Fmt> MatchBase for DynMatchAdapter<M, Fmt>
where
    M: MatchBase,
    Fmt: ResultFormat,
{
    type In = M::In;
}

impl<M, Fmt> DynMatchPos for DynMatchAdapter<M, Fmt>
where
    M: MatchPos,
    Fmt: ResultFormat<Pos = M::PosFail>,
{
    type PosOut = M::PosOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        match self.matcher.match_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(DynMatchFailure::new(
                MatchFailure::Pos(result),
                self.format,
            )?)),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMatchNeg for DynMatchAdapter<M, Fmt>
where
    M: MatchNeg,
    Fmt: ResultFormat<Neg = M::NegFail>,
{
    type NegOut = M::NegOut;

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        match self.matcher.match_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(DynMatchFailure::new(
                MatchFailure::Neg(result),
                self.format,
            )?)),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub(super) struct SimpleMatchAdapter<M, Actual>
where
    M: SimpleMatch<Actual>,
{
    inner: M,
    marker: PhantomData<Actual>,
}

impl<M, Actual> SimpleMatchAdapter<M, Actual>
where
    M: SimpleMatch<Actual>,
{
    pub fn new(inner: M) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<M, Actual> MatchBase for SimpleMatchAdapter<M, Actual>
where
    M: SimpleMatch<Actual>,
{
    type In = Actual;
}

impl<M, Actual> MatchPos for SimpleMatchAdapter<M, Actual>
where
    M: SimpleMatch<Actual>,
{
    type PosOut = Actual;
    type PosFail = M::Fail;

    fn match_pos(
        mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match self.inner.matches(actual.borrow()) {
            Ok(true) => Ok(MatchResult::Success(actual)),
            Ok(false) => Ok(MatchResult::Fail(self.inner.fail(actual))),
            Err(error) => Err(error),
        }
    }
}

impl<M, Actual> MatchNeg for SimpleMatchAdapter<M, Actual>
where
    M: SimpleMatch<Actual>,
{
    type NegOut = Actual;
    type NegFail = M::Fail;

    fn match_neg(
        mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match self.inner.matches(actual.borrow()) {
            Ok(true) => Ok(MatchResult::Fail(self.inner.fail(actual))),
            Ok(false) => Ok(MatchResult::Success(actual)),
            Err(error) => Err(error),
        }
    }
}
