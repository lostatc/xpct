use std::fmt;

use super::{matcher::Match, BoxMatch, FormattedFailure, MatchOutcome, Matcher};

pub(super) struct MatchWrapper<'a, In, PosOut, NegOut> {
    inner: BoxMatch<'a, In, PosOut, NegOut>,
}

impl<'a, In, PosOut, NegOut> MatchWrapper<'a, In, PosOut, NegOut> {
    pub fn new(matcher: Matcher<'a, In, PosOut, NegOut>) -> Self {
        Self {
            inner: matcher.into_box(),
        }
    }
}

impl<'a, In, PosOut, NegOut> fmt::Debug for MatchWrapper<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MatchWrapper").finish_non_exhaustive()
    }
}

impl<'a, In, PosOut, NegOut> Match for MatchWrapper<'a, In, PosOut, NegOut> {
    type In = In;

    type PosOut = PosOut;
    type NegOut = NegOut;

    type PosFail = FormattedFailure;
    type NegFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.inner.match_pos(actual)
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.inner.match_neg(actual)
    }
}
