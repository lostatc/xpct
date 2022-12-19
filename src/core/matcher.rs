use std::fmt;

use super::adapter::{DynMatchAdapter, NegMatchAdapter, SimpleMatchAdapter};
use super::wrap::MatchWrapper;
use super::{FormattedFailure, MatchOutcome, ResultFormat};

pub trait Match {
    type In;

    type PosOut;
    type NegOut;

    type PosFail;
    type NegFail;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>>;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>>;
}

pub trait SimpleMatch<Actual> {
    type Fail;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool>;

    fn fail(self, actual: Actual) -> Self::Fail;
}

pub trait DynMatch {
    type In;

    type PosOut;
    type NegOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>>;

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>>;
}

pub type BoxMatch<'a, In, PosOut, NegOut = PosOut> =
    Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a>;

pub struct Matcher<'a, In, PosOut, NegOut = PosOut> {
    inner: BoxMatch<'a, In, PosOut, NegOut>,
}

impl<'a, In, PosOut, NegOut> fmt::Debug for Matcher<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matcher").finish_non_exhaustive()
    }
}

impl<'a, In, PosOut, NegOut> Matcher<'a, In, PosOut, NegOut> {
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: Match<In = In, PosOut = PosOut, NegOut = NegOut> + 'a,
        Fmt: ResultFormat<Pos = M::PosFail, Neg = M::NegFail> + 'a,
    {
        Self {
            inner: Box::new(DynMatchAdapter::new(matcher, format)),
        }
    }

    pub fn neg<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: Match<In = In, PosOut = NegOut, NegOut = PosOut> + 'a,
        Fmt: ResultFormat<Pos = M::NegFail, Neg = M::PosFail> + 'a,
    {
        Matcher::new(NegMatchAdapter::new(matcher), format)
    }

    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        PosOut: 'a,
        NegOut: 'a,
        Fmt: ResultFormat<Pos = FormattedFailure, Neg = FormattedFailure> + 'a,
    {
        Self::new(MatchWrapper::new(self), format)
    }

    pub fn into_box(self) -> BoxMatch<'a, In, PosOut, NegOut> {
        self.inner
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

impl<'a, In, PosOut, NegOut> DynMatch for Matcher<'a, In, PosOut, NegOut> {
    type In = In;

    type PosOut = PosOut;
    type NegOut = NegOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>> {
        self.inner.match_pos(actual)
    }

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>> {
        self.inner.match_neg(actual)
    }
}
