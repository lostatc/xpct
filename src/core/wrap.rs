use std::any::type_name;
use std::fmt;

use super::{
    BoxMatch, BoxMatchNeg, BoxMatchPos, DynMatchFailure, MatchBase, MatchNeg, MatchPos,
    MatchResult, Matcher, NegMatcher, PosMatcher,
};

pub(super) struct MatchWrapper<'a, In, PosOut, NegOut>(BoxMatch<'a, In, PosOut, NegOut>);

impl<'a, In, PosOut, NegOut> MatchWrapper<'a, In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<'a, In, PosOut, NegOut>) -> Self {
        Self(matcher.into_box())
    }
}

impl<'a, In, PosOut, NegOut> fmt::Debug for MatchWrapper<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchWrapper")
            .field(&type_name::<BoxMatch<'a, In, PosOut, NegOut>>())
            .finish()
    }
}

impl<'a, In, PosOut, NegOut> MatchBase for MatchWrapper<'a, In, PosOut, NegOut> {
    type In = In;
}

impl<'a, In, PosOut, NegOut> MatchPos for MatchWrapper<'a, In, PosOut, NegOut> {
    type PosOut = PosOut;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.match_pos(actual)
    }
}

impl<'a, In, PosOut, NegOut> MatchNeg for MatchWrapper<'a, In, PosOut, NegOut> {
    type NegOut = NegOut;
    type NegFail = DynMatchFailure;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        self.0.match_neg(actual)
    }
}

pub(super) struct MatchPosWrapper<'a, In, Out>(BoxMatchPos<'a, In, Out>);

impl<'a, In, Out> MatchPosWrapper<'a, In, Out> {
    pub fn new(matcher: PosMatcher<'a, In, Out>) -> Self {
        Self(matcher.into_box())
    }
}

impl<'a, In, Out> fmt::Debug for MatchPosWrapper<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchPosWrapper")
            .field(&type_name::<BoxMatchPos<'a, In, Out>>())
            .finish()
    }
}

impl<'a, In, Out> MatchBase for MatchPosWrapper<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for MatchPosWrapper<'a, In, Out> {
    type PosOut = Out;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.match_pos(actual)
    }
}

pub(super) struct MatchNegWrapper<'a, In, Out>(BoxMatchNeg<'a, In, Out>);

impl<'a, In, Out> MatchNegWrapper<'a, In, Out> {
    pub fn new(matcher: NegMatcher<'a, In, Out>) -> Self {
        Self(matcher.into_box())
    }
}

impl<'a, In, Out> fmt::Debug for MatchNegWrapper<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchNegWrapper")
            .field(&type_name::<BoxMatchNeg<'a, In, Out>>())
            .finish()
    }
}

impl<'a, In, Out> MatchBase for MatchNegWrapper<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchNeg for MatchNegWrapper<'a, In, Out> {
    type NegOut = Out;
    type NegFail = DynMatchFailure;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        self.0.match_neg(actual)
    }
}
