use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;

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
    /// Create a new [`MapMatcher`].
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

    type PosFail = Infallible;
    type NegFail = Infallible;

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
    /// Create a new [`TryMapMatcher`].
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

/// The matcher for [`iter_map`].
///
/// [`iter_map`]: crate::iter_map
pub struct IterMapMatcher<'a, In, Out, IntoIter> {
    func: Box<dyn Fn(In) -> Out + 'a>,
    marker: PhantomData<IntoIter>,
}

impl<'a, In, Out, IntoIter> fmt::Debug for IterMapMatcher<'a, In, Out, IntoIter> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IterMapMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out, IntoIter> IterMapMatcher<'a, In, Out, IntoIter> {
    /// Create a new [`IterMapMatcher`].
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(In) -> Out + 'a,
    {
        Self {
            func: Box::new(func),
            marker: PhantomData,
        }
    }
}

impl<'a, In, Out, IntoIter> Match for IterMapMatcher<'a, In, Out, IntoIter>
where
    IntoIter: IntoIterator<Item = In> + 'a,
{
    type In = IntoIter;

    type PosOut = Vec<Out>;
    type NegOut = Vec<Out>;

    type PosFail = Infallible;
    type NegFail = Infallible;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!(actual.into_iter().map(self.func).collect::<Vec<_>>())
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        success!(actual.into_iter().map(self.func).collect::<Vec<_>>())
    }
}

/// The matcher for [`iter_try_map`].
///
/// [`iter_try_map`]: crate::iter_try_map
pub struct IterTryMapMatcher<'a, In, Out, IntoIter> {
    func: Box<dyn Fn(In) -> crate::Result<Out> + 'a>,
    marker: PhantomData<IntoIter>,
}

impl<'a, In, Out, IntoIter> fmt::Debug for IterTryMapMatcher<'a, In, Out, IntoIter> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IterTryMapMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out, IntoIter> IterTryMapMatcher<'a, In, Out, IntoIter> {
    /// Create a new [`IterTryMapMatcher`].
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(In) -> crate::Result<Out> + 'a,
    {
        Self {
            func: Box::new(func),
            marker: PhantomData,
        }
    }
}

impl<'a, In, Out, IntoIter> Match for IterTryMapMatcher<'a, In, Out, IntoIter>
where
    IntoIter: IntoIterator<Item = In> + 'a,
{
    type In = IntoIter;

    type PosOut = Vec<Out>;
    type NegOut = Vec<Out>;

    type PosFail = Infallible;
    type NegFail = Infallible;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!(actual
            .into_iter()
            .map(self.func)
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        success!(actual
            .into_iter()
            .map(self.func)
            .collect::<Result<Vec<_>, _>>()?)
    }
}
