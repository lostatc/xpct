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
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>>;
}

pub trait MatchNeg: MatchBase {
    type NegOut;
    type NegFail;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>>;
}

pub trait SimpleMatch {
    type Value;
    type Fail;

    fn matches(&mut self, actual: &Self::Value) -> anyhow::Result<bool>;

    fn fail(self, actual: Self::Value) -> Self::Fail;
}

pub trait DynMatchPos: MatchBase {
    type PosOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>>;
}

pub trait DynMatchNeg: MatchBase {
    type NegOut;

    fn match_neg(
        self: Box<Self>,
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
    Fmt: ResultFormat<Pos = M::PosFail>,
{
    type PosOut = M::PosOut;

    fn match_pos(
        self: Box<Self>,
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
    Fmt: ResultFormat<Neg = M::NegFail>,
{
    type NegOut = M::NegOut;

    fn match_neg(
        self: Box<Self>,
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
    Fmt: ResultFormat<Pos = M::PosFail, Neg = M::NegFail>,
{
}

impl<T> MatchBase for T
where
    T: SimpleMatch,
{
    type In = <Self as SimpleMatch>::Value;
}

impl<T> MatchPos for T
where
    T: SimpleMatch,
{
    type PosOut = <Self as SimpleMatch>::Value;
    type PosFail = <Self as SimpleMatch>::Fail;

    fn match_pos(
        mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match self.matches(&actual) {
            Ok(true) => Ok(MatchResult::Success(actual)),
            Ok(false) => Ok(MatchResult::Fail(self.fail(actual))),
            Err(error) => Err(error),
        }
    }
}

impl<T> MatchNeg for T
where
    T: SimpleMatch,
{
    type NegOut = <T as SimpleMatch>::Value;
    type NegFail = <T as SimpleMatch>::Fail;

    fn match_neg(
        mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match self.matches(&actual) {
            Ok(true) => Ok(MatchResult::Fail(self.fail(actual))),
            Ok(false) => Ok(MatchResult::Success(actual)),
            Err(error) => Err(error),
        }
    }
}

pub type BoxMatcher<In, PosOut, NegOut> = Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut>>;

pub struct Matcher<In, PosOut, NegOut = PosOut>(BoxMatcher<In, PosOut, NegOut>);

impl<In, PosOut, NegOut> fmt::Debug for Matcher<In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matcher").finish()
    }
}

impl<In, PosOut, NegOut> Matcher<In, PosOut, NegOut> {
    pub fn new<M, Fmt>(matcher: M) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = PosOut> + MatchNeg<NegOut = NegOut> + 'static,
        Fmt: ResultFormat<Pos = M::PosFail, Neg = M::NegFail>,
    {
        Self(Box::new(InnerMatcher::<_, Fmt>::new(matcher)))
    }

    pub fn into_box(self) -> BoxMatcher<In, PosOut, NegOut> {
        self.0
    }
}

impl<In, PosOut, NegOut> MatchBase for Matcher<In, PosOut, NegOut> {
    type In = In;
}

impl<In, PosOut, NegOut> DynMatchPos for Matcher<In, PosOut, NegOut> {
    type PosOut = PosOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        self.0.match_pos(actual)
    }
}

impl<In, PosOut, NegOut> DynMatchNeg for Matcher<In, PosOut, NegOut>
{
    type NegOut = NegOut;

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        self.0.match_neg(actual)
    }
}


impl<In, PosOut, NegOut> DynMatch for Matcher<In, PosOut, NegOut> {}
