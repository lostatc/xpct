use std::fmt;
use std::marker::PhantomData;

use super::format::ResultFormat;
use super::result::{DynMatchFailure, MatchResult, MatchFailure};

pub trait MatchBase {
    type In;
}

pub trait MatchPos: MatchBase {
    type PosOut;
    type PosFail;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>>;
}

pub trait MatchNeg: MatchBase {
    type NegOut;
    type NegFail;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>>;
}

pub trait DynMatchPos: MatchBase {
    type PosOut;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>>;
}

pub trait DynMatchNeg: MatchBase {
    type NegOut;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>>;
}

pub trait DynMatch: DynMatchPos + DynMatchNeg {}

struct InnerMatcher<M, Fmt: ResultFormat> {
    matcher: M,
    result_fmt: PhantomData<Fmt>,
}

impl<M, Fmt: ResultFormat> InnerMatcher<M, Fmt> {
    pub fn new(matcher: M) -> Self {
        Self {
            matcher,
            result_fmt: PhantomData,
        }
    }
}

impl<M, Fmt> MatchBase for InnerMatcher<M, Fmt>
where
    M: MatchBase,
    Fmt: ResultFormat,
{
    type In = M::In;
}

impl<M, Fmt> DynMatchPos for InnerMatcher<M, Fmt>
where
    M: MatchPos,
    Fmt: ResultFormat<PosFail = M::PosFail>,
{
    type PosOut = M::PosOut;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        match self.matcher.match_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(DynMatchFailure::new::<Fmt, _, _>(MatchFailure::Pos(result)))),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMatchNeg for InnerMatcher<M, Fmt>
where
    M: MatchNeg,
    Fmt: ResultFormat<NegFail = M::NegFail>,
{
    type NegOut = M::NegOut;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        match self.matcher.match_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(DynMatchFailure::new::<Fmt, _, _>(MatchFailure::Neg(result)))),
            Err(error) => Err(error),
        }
    }
}

impl<M, Fmt> DynMatch for InnerMatcher<M, Fmt>
where
    M: MatchPos + MatchNeg,
    Fmt: ResultFormat<PosFail = M::PosFail, NegFail = M::NegFail>,
{
}

pub struct Matcher<'a, In, PosOut, NegOut>(
    Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a>,
);

impl<'a, In, PosOut, NegOut> fmt::Debug for Matcher<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matcher").finish()
    }
}

impl<'a, In, PosOut, NegOut> Matcher<'a, In, PosOut, NegOut> {
    pub fn new<M, Fmt>(matcher: M) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = PosOut> + MatchNeg<NegOut = NegOut> + 'a,
        Fmt: ResultFormat<PosFail = M::PosFail, NegFail = M::NegFail>,
    {
        Self(Box::new(InnerMatcher::<_, Fmt>::new(matcher)))
    }
}

impl<'a, In, PosOut, NegOut> MatchBase for Matcher<'a, In, PosOut, NegOut> {
    type In = In;
}

impl<'a, In, PosOut, NegOut> DynMatchPos for Matcher<'a, In, PosOut, NegOut> {
    type PosOut = PosOut;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        self.0.match_pos(actual)
    }
}

impl<'a, In, PosOut, NegOut> DynMatchNeg for Matcher<'a, In, PosOut, NegOut>
{
    type NegOut = NegOut;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        self.0.match_neg(actual)
    }
}


impl<'a, In, PosOut, NegOut> DynMatch for Matcher<'a, In, PosOut, NegOut> {}
