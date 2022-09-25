use std::{borrow::Borrow, marker::PhantomData};

use super::{
    DynMatchNeg, DynMatchPos, FormattedFailure, MatchBase, MatchFailure, MatchNeg, MatchOutcome,
    MatchPos, ResultFormat, SimpleMatch,
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
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>> {
        match self.matcher.match_pos(actual) {
            Ok(MatchOutcome::Success(out)) => Ok(MatchOutcome::Success(out)),
            Ok(MatchOutcome::Fail(result)) => Ok(MatchOutcome::Fail(FormattedFailure::new(
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
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>> {
        match self.matcher.match_neg(actual) {
            Ok(MatchOutcome::Success(out)) => Ok(MatchOutcome::Success(out)),
            Ok(MatchOutcome::Fail(result)) => Ok(MatchOutcome::Fail(FormattedFailure::new(
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
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match self.inner.matches(actual.borrow()) {
            Ok(true) => Ok(MatchOutcome::Success(actual)),
            Ok(false) => Ok(MatchOutcome::Fail(self.inner.fail(actual))),
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
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match self.inner.matches(actual.borrow()) {
            Ok(true) => Ok(MatchOutcome::Fail(self.inner.fail(actual))),
            Ok(false) => Ok(MatchOutcome::Success(actual)),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub(super) struct NegMatchAdapter<M> {
    matcher: M,
}

impl<M> NegMatchAdapter<M> {
    pub fn new(matcher: M) -> Self {
        Self { matcher }
    }
}

impl<M> MatchBase for NegMatchAdapter<M>
where
    M: MatchBase,
{
    type In = M::In;
}

impl<M> MatchPos for NegMatchAdapter<M>
where
    M: MatchNeg,
{
    type PosOut = M::NegOut;
    type PosFail = M::NegFail;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.matcher.match_neg(actual)
    }
}

impl<M> MatchNeg for NegMatchAdapter<M>
where
    M: MatchPos,
{
    type NegOut = M::PosOut;
    type NegFail = M::PosFail;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.matcher.match_pos(actual)
    }
}
