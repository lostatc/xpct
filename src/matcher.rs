use std::fmt;
use std::marker::PhantomData;

use super::format::ResultFormat;
use super::result::{MatchFailure, MatchResult};

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
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>>;
}

pub trait DynMatchNeg: MatchBase {
    type NegOut;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>>;
}

pub trait DynMatch: DynMatchPos + DynMatchNeg {}

pub struct InnerMatcher<M, Fmt: ResultFormat> {
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
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>> {
        match self.matcher.match_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(MatchFailure::new_pos::<
                M::PosFail,
                Fmt,
            >(result))),
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
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>> {
        match self.matcher.match_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(MatchFailure::new_neg::<
                M::NegFail,
                Fmt,
            >(result))),
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

pub struct Matcher<In, PosOut, NegOut>(
    Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut>>,
);

impl<In, PosOut, NegOut> fmt::Debug for Matcher<In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matcher").finish()
    }
}

impl<In, PosOut, NegOut> Matcher<In, PosOut, NegOut> {
    pub fn new<M, Fmt>(matcher: M) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = PosOut> + MatchNeg<NegOut = NegOut> + 'static,
        Fmt: ResultFormat<PosFail = M::PosFail, NegFail = M::NegFail>,
    {
        Self(Box::new(InnerMatcher::<_, Fmt>::new(matcher)))
    }
}

impl<In, PosOut, NegOut> MatchBase for Matcher<In, PosOut, NegOut> {
    type In = In;
}

impl<In, PosOut, NegOut> DynMatchPos for Matcher<In, PosOut, NegOut> {
    type PosOut = PosOut;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, MatchFailure>> {
        self.0.match_pos(actual)
    }
}

impl<In, PosOut, NegOut> DynMatchNeg for Matcher<In, PosOut, NegOut>
{
    type NegOut = NegOut;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, MatchFailure>> {
        self.0.match_neg(actual)
    }
}


impl<In, PosOut, NegOut> DynMatch for Matcher<In, PosOut, NegOut> {}
