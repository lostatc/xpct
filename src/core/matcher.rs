use std::any::type_name;
use std::borrow::Borrow;
use std::fmt;
use std::marker::PhantomData;

use super::wrap::{MatchNegWrapper, MatchPosWrapper, MatchWrapper};
use super::{DynMatchFailure, MatchFailure, MatchResult, ResultFormat};

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

pub trait SimpleMatch<Actual> {
    type Fail;

    fn matches(&mut self, actual: &Actual) -> anyhow::Result<bool>;

    fn fail(self, actual: Actual) -> Self::Fail;
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

impl<T> DynMatch for T where T: DynMatchPos + DynMatchNeg {}

#[derive(Debug)]
struct DynMatchAdapter<M, Fmt: ResultFormat> {
    matcher: M,
    format: Fmt,
}

impl<M, Fmt: ResultFormat> DynMatchAdapter<M, Fmt> {
    fn new(matcher: M, format: Fmt) -> Self {
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
struct SimpleMatchAdapter<M, Actual>
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
    fn new(inner: M) -> Self {
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

pub type BoxMatch<'a, In, PosOut, NegOut = PosOut> =
    Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a>;

pub type BoxMatchPos<'a, In, PosOut> = Box<dyn DynMatchPos<In = In, PosOut = PosOut> + 'a>;

pub type BoxMatchNeg<'a, In, NegOut> = Box<dyn DynMatchNeg<In = In, NegOut = NegOut> + 'a>;

pub struct Matcher<'a, In, PosOut, NegOut = PosOut>(BoxMatch<'a, In, PosOut, NegOut>);

impl<'a, In, PosOut, NegOut> fmt::Debug for Matcher<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Matcher")
            .field(&type_name::<BoxMatch<'a, In, PosOut, NegOut>>())
            .finish()
    }
}

impl<'a, In, PosOut, NegOut> Matcher<'a, In, PosOut, NegOut> {
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = PosOut> + MatchNeg<NegOut = NegOut> + 'a,
        Fmt: ResultFormat<Pos = M::PosFail, Neg = M::NegFail> + 'a,
    {
        Self(Box::new(DynMatchAdapter::new(matcher, format)))
    }

    pub fn wrap<Fmt>(matcher: Matcher<'a, In, PosOut, NegOut>, format: Fmt) -> Self
    where
        In: 'a,
        PosOut: 'a,
        NegOut: 'a,
        Fmt: ResultFormat<Pos = DynMatchFailure, Neg = DynMatchFailure> + 'a,
    {
        Self::new(MatchWrapper::new(matcher), format)
    }

    pub fn into_box(self) -> BoxMatch<'a, In, PosOut, NegOut> {
        self.0
    }
}

impl<'a, Actual> Matcher<'a, Actual, Actual> {
    pub fn simple<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: SimpleMatch<Actual> + 'a,
        Fmt: ResultFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
        Actual: 'a,
    {
        Self::new(SimpleMatchAdapter::new(matcher), format)
    }
}

impl<'a, In, PosOut, NegOut> MatchBase for Matcher<'a, In, PosOut, NegOut> {
    type In = In;
}

impl<'a, In, PosOut, NegOut> DynMatchPos for Matcher<'a, In, PosOut, NegOut> {
    type PosOut = PosOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        self.0.match_pos(actual)
    }
}

impl<'a, In, PosOut, NegOut> DynMatchNeg for Matcher<'a, In, PosOut, NegOut> {
    type NegOut = NegOut;

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        self.0.match_neg(actual)
    }
}

pub struct PosMatcher<'a, In, Out>(BoxMatchPos<'a, In, Out>);

impl<'a, In, Out> fmt::Debug for PosMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PosMatcher")
            .field(&type_name::<BoxMatchPos<'a, In, Out>>())
            .finish()
    }
}

impl<'a, In, Out> PosMatcher<'a, In, Out> {
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = Out> + 'a,
        Fmt: ResultFormat<Pos = M::PosFail> + 'a,
    {
        Self(Box::new(DynMatchAdapter::new(matcher, format)))
    }

    pub fn wrap<Fmt>(matcher: PosMatcher<'a, In, Out>, format: Fmt) -> Self
    where
        In: 'a,
        Out: 'a,
        Fmt: ResultFormat<Pos = DynMatchFailure> + 'a,
    {
        Self::new(MatchPosWrapper::new(matcher), format)
    }

    pub fn into_box(self) -> BoxMatchPos<'a, In, Out> {
        self.0
    }
}

impl<'a, In, Out> MatchBase for PosMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> DynMatchPos for PosMatcher<'a, In, Out> {
    type PosOut = Out;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, DynMatchFailure>> {
        self.0.match_pos(actual)
    }
}

pub struct NegMatcher<'a, In, Out>(BoxMatchNeg<'a, In, Out>);

impl<'a, In, Out> fmt::Debug for NegMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NegMatcher")
            .field(&type_name::<BoxMatchNeg<'a, In, Out>>())
            .finish()
    }
}

impl<'a, In, Out> NegMatcher<'a, In, Out> {
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: MatchBase<In = In> + MatchNeg<NegOut = Out> + 'a,
        Fmt: ResultFormat<Neg = M::NegFail> + 'a,
    {
        Self(Box::new(DynMatchAdapter::new(matcher, format)))
    }

    pub fn wrap<Fmt>(matcher: NegMatcher<'a, In, Out>, format: Fmt) -> Self
    where
        In: 'a,
        Out: 'a,
        Fmt: ResultFormat<Neg = DynMatchFailure> + 'a,
    {
        Self::new(MatchNegWrapper::new(matcher), format)
    }

    pub fn into_box(self) -> BoxMatchNeg<'a, In, Out> {
        self.0
    }
}

impl<'a, In, Out> MatchBase for NegMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> DynMatchNeg for NegMatcher<'a, In, Out> {
    type NegOut = Out;

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, DynMatchFailure>> {
        self.0.match_neg(actual)
    }
}
