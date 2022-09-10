use std::fmt;

use crate::{
    DynMatchFailure, MatchBase, MatchFailure, MatchNeg, MatchPos, MatchResult, Matcher,
    ResultFormat,
};

#[derive(Debug)]
pub struct NotFormat(MatchFailure<DynMatchFailure>);

impl fmt::Display for NotFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl ResultFormat for NotFormat {
    type Pos = DynMatchFailure;
    type Neg = DynMatchFailure;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

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

pub fn not<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
) -> Matcher<In, NegOut, PosOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::new::<NotFormat, _>(NotMatcher::new(matcher))
}
