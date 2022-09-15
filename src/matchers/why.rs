use crate::core::{DynMatchFailure, MatchBase, MatchNeg, MatchPos, MatchResult, Matcher};

#[derive(Debug)]
pub struct WhyMatcher<'a, In, PosOut, NegOut>(Matcher<'a, In, PosOut, NegOut>);

impl<'a, In, PosOut, NegOut> WhyMatcher<'a, In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<'a, In, PosOut, NegOut>) -> Self {
        Self(matcher)
    }
}

impl<'a, In, PosOut, NegOut> MatchBase for WhyMatcher<'a, In, PosOut, NegOut> {
    type In = In;
}

impl<'a, In, PosOut, NegOut> MatchPos for WhyMatcher<'a, In, PosOut, NegOut> {
    type PosOut = PosOut;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.into_box().match_pos(actual)
    }
}

impl<'a, In, PosOut, NegOut> MatchNeg for WhyMatcher<'a, In, PosOut, NegOut> {
    type NegOut = NegOut;
    type NegFail = DynMatchFailure;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        self.0.into_box().match_neg(actual)
    }
}
