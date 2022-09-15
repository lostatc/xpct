use crate::core::{DynMatchFailure, MatchBase, MatchNeg, MatchPos, MatchResult, Matcher};

#[derive(Debug)]
pub struct NotMatcher<'a, In, PosOut, NegOut>(Matcher<'a, In, PosOut, NegOut>);

impl<'a, In, PosOut, NegOut> NotMatcher<'a, In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<'a, In, PosOut, NegOut>) -> Self {
        NotMatcher(matcher)
    }
}

impl<'a, In, PosOut, NegOut> MatchBase for NotMatcher<'a, In, PosOut, NegOut> {
    type In = In;
}

impl<'a, In, PosOut, NegOut> MatchPos for NotMatcher<'a, In, PosOut, NegOut> {
    type PosOut = NegOut;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.into_box().match_neg(actual)
    }
}

impl<'a, In, PosOut, NegOut> MatchNeg for NotMatcher<'a, In, PosOut, NegOut> {
    type NegOut = PosOut;
    type NegFail = DynMatchFailure;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        self.0.into_box().match_pos(actual)
    }
}
