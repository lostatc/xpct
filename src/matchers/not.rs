use crate::core::{FormattedFailure, Match, MatchOutcome, Matcher};

/// The matcher for [`not`]
///
/// [`not`]: crate::not
#[derive(Debug)]
pub struct NotMatcher<'a, In, PosOut, NegOut> {
    inner: Matcher<'a, In, PosOut, NegOut>,
}

impl<'a, In, PosOut, NegOut> NotMatcher<'a, In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<'a, In, PosOut, NegOut>) -> Self {
        NotMatcher { inner: matcher }
    }
}

impl<'a, In, PosOut, NegOut> Match for NotMatcher<'a, In, PosOut, NegOut> {
    type In = In;

    type PosOut = NegOut;
    type NegOut = PosOut;

    type PosFail = FormattedFailure;
    type NegFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.inner.into_box().match_neg(actual)
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.inner.into_box().match_pos(actual)
    }
}
