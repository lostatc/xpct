use std::fmt;

use crate::core::{FormattedFailure, Match, MatchOutcome};
use crate::success;

/// The matcher for [`map`].
///
/// [`map`]: crate::map
pub struct MapMatcher<'a, In, Out> {
    func: Box<dyn FnOnce(In) -> Out + 'a>,
}

impl<'a, In, Out> fmt::Debug for MapMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out> MapMatcher<'a, In, Out> {
    pub fn new<F>(func: F) -> Self
    where
        F: FnOnce(In) -> Out + 'a,
    {
        Self {
            func: Box::new(func),
        }
    }
}

impl<'a, In, Out> Match for MapMatcher<'a, In, Out> {
    type In = In;

    type PosOut = Out;
    type NegOut = Out;

    type PosFail = FormattedFailure;
    type NegFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual))
    }
}

/// The matcher for [`try_map`].
///
/// [`try_map`]: crate::map
pub struct TryMapMatcher<'a, In, Out> {
    func: Box<dyn FnOnce(In) -> crate::Result<Out> + 'a>,
}

impl<'a, In, Out> fmt::Debug for TryMapMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TryMapMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out> TryMapMatcher<'a, In, Out> {
    pub fn new(func: impl FnOnce(In) -> crate::Result<Out> + 'a) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

impl<'a, In, Out> Match for TryMapMatcher<'a, In, Out> {
    type In = In;

    type PosOut = Out;
    type NegOut = Out;

    type PosFail = FormattedFailure;
    type NegFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual)?)
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual)?)
    }
}
