use crate::{
    Format, Formatter, MatchNeg, MatchPos, MatchBase, DynMatchNeg, DynMatchPos, DynMatchFailure, MatchResult, Matcher,
    ResultFormat, MatchFailure,
};

#[derive(Debug)]
pub struct NotFormat(MatchFailure<DynMatchFailure, DynMatchFailure>);

impl Format for NotFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl From<MatchFailure<DynMatchFailure, DynMatchFailure>> for NotFormat {
    fn from(result: MatchFailure<DynMatchFailure, DynMatchFailure>) -> Self {
        Self(result)
    }
}

impl ResultFormat for NotFormat {
    type PosFail = DynMatchFailure;
    type NegFail = DynMatchFailure;
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
    type PosFail = DynMatchFailure;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        self.0.match_neg(actual)
    }
}

impl<In, PosOut, NegOut> MatchNeg for NotMatcher<In, PosOut, NegOut> {
    type NegOut = PosOut;
    type NegFail = DynMatchFailure;

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
