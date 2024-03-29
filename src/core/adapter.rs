use std::marker::PhantomData;

use super::{
    DynTransformMatch, FormattedFailure, Match, MatchFailure, MatchOutcome, MatcherFormat,
    TransformMatch,
};

#[derive(Debug)]
pub(super) struct DynTransformMatchAdapter<M, Fmt: MatcherFormat> {
    matcher: M,
    format: Fmt,
}

impl<M, Fmt: MatcherFormat> DynTransformMatchAdapter<M, Fmt> {
    pub fn new(matcher: M, format: Fmt) -> Self {
        Self { matcher, format }
    }
}

impl<M, Fmt> DynTransformMatch for DynTransformMatchAdapter<M, Fmt>
where
    M: TransformMatch,
    Fmt: MatcherFormat<Pos = M::PosFail, Neg = M::NegFail>,
{
    type In = M::In;

    type PosOut = M::PosOut;
    type NegOut = M::NegOut;

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
pub(super) struct MatchAdapter<M, Actual>
where
    M: Match<Actual>,
{
    inner: M,
    marker: PhantomData<Actual>,
}

impl<M, Actual> MatchAdapter<M, Actual>
where
    M: Match<Actual>,
{
    pub fn new(inner: M) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<M, Actual> TransformMatch for MatchAdapter<M, Actual>
where
    M: Match<Actual>,
{
    type In = Actual;

    type PosOut = Actual;
    type NegOut = Actual;

    type PosFail = M::Fail;
    type NegFail = M::Fail;

    fn match_pos(
        mut self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match self.inner.matches(&actual) {
            Ok(true) => Ok(MatchOutcome::Success(actual)),
            Ok(false) => Ok(MatchOutcome::Fail(self.inner.fail(actual))),
            Err(error) => Err(error),
        }
    }

    fn match_neg(
        mut self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match self.inner.matches(&actual) {
            Ok(true) => Ok(MatchOutcome::Fail(self.inner.fail(actual))),
            Ok(false) => Ok(MatchOutcome::Success(actual)),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub(super) struct NegTransformMatchAdapter<M> {
    matcher: M,
}

impl<M> NegTransformMatchAdapter<M> {
    pub fn new(matcher: M) -> Self {
        Self { matcher }
    }
}

impl<M> TransformMatch for NegTransformMatchAdapter<M>
where
    M: TransformMatch,
{
    type In = M::In;

    type PosOut = M::NegOut;
    type NegOut = M::PosOut;

    type PosFail = M::NegFail;
    type NegFail = M::PosFail;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.matcher.match_neg(actual)
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.matcher.match_pos(actual)
    }
}
