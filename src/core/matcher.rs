use std::any::type_name;
use std::fmt;

use super::adapter::{DynMatchAdapter, NegMatchAdapter, SimpleMatchAdapter};
use super::wrap::{MatchNegWrapper, MatchPosWrapper, MatchWrapper};
use super::{DynMatchFailure, MatchResult, ResultFormat};

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

    pub fn neg<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: MatchBase<In = In> + MatchPos<PosOut = NegOut> + MatchNeg<NegOut = PosOut> + 'a,
        Fmt: ResultFormat<Pos = M::NegFail, Neg = M::PosFail> + 'a,
    {
        Matcher::new(NegMatchAdapter::new(matcher), format)
    }

    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        PosOut: 'a,
        NegOut: 'a,
        Fmt: ResultFormat<Pos = DynMatchFailure, Neg = DynMatchFailure> + 'a,
    {
        Self::new(MatchWrapper::new(self), format)
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

    pub fn simple_neg<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: SimpleMatch<Actual> + 'a,
        Fmt: ResultFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
        Actual: 'a,
    {
        Self::neg(SimpleMatchAdapter::new(matcher), format)
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

    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        Out: 'a,
        Fmt: ResultFormat<Pos = DynMatchFailure> + 'a,
    {
        Self::new(MatchPosWrapper::new(self), format)
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

    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        Out: 'a,
        Fmt: ResultFormat<Neg = DynMatchFailure> + 'a,
    {
        Self::new(MatchNegWrapper::new(self), format)
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
