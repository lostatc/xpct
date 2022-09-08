use crate::{
    Format, Formatter, MatchNeg, MatchPos, MatchBase, DynMatchNeg, DynMatchPos, MatchFailure, MatchResult, Matcher,
    ResultFormat,
};

#[derive(Debug)]
pub struct NotFormat(MatchResult<MatchFailure, MatchFailure>);

impl Format for NotFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl From<MatchResult<MatchFailure, MatchFailure>> for NotFormat {
    fn from(result: MatchResult<MatchFailure, MatchFailure>) -> Self {
        Self(result)
    }
}

impl ResultFormat for NotFormat {
    type PosFail = MatchFailure;
    type NegFail = MatchFailure;
}

#[derive(Debug)]
pub struct NotMatcher<In, PosOut, NegOut>(Matcher<In, PosOut, NegOut>);

impl<In, PosOut, NegOut> NotMatcher<In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<In, PosOut, NegOut>) -> Self {
        NotMatcher(matcher)
    }
}

impl<In, PosOut, NegOut> MatchBase for NotMatcher<In, PosOut, NegOut> {
    type In = In;
}

impl<In, PosOut, NegOut> MatchPos for NotMatcher<In, PosOut, NegOut> {
    type PosOut = NegOut;
    type PosFail = MatchFailure;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.match_neg(actual)
    }
}

impl<In, PosOut, NegOut> MatchNeg for NotMatcher<In, PosOut, NegOut> {
    type NegOut = PosOut;
    type NegFail = MatchFailure;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        self.0.match_pos(actual)
    }
}

pub fn not<In, PosOut, NegOut>(matcher: Matcher<In, PosOut, NegOut>) -> Matcher<In, NegOut, PosOut>
where
    In: 'static,
    PosOut: 'static,
    NegOut: 'static,
{
    Matcher::new::<_, NotFormat>(NotMatcher::new(matcher))
}
