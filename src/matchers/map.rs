use std::convert::Infallible;
use std::fmt;
use std::iter;
use std::marker::PhantomData;

use crate::core::{FormattedFailure, Match, MatchOutcome};

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
    pub fn new(func: impl FnOnce(In) -> Out + 'a) -> Self {
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
        Ok(MatchOutcome::Success((self.func)(actual)))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.match_pos(actual)
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
        Ok(MatchOutcome::Success((self.func)(actual)?))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        self.match_pos(actual)
    }
}

/// An iterator returned by [`IterMapMatcher`].
pub struct IterMap<'a, In, Out, I> {
    inner: I,
    func: Box<dyn Fn(In) -> Out + 'a>,
}

impl<'a, In, Out, I> fmt::Debug for IterMap<'a, In, Out, I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IterMap")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<'a, In, Out, I> Iterator for IterMap<'a, In, Out, I>
where
    I: Iterator<Item = In>,
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(&self.func)
    }
}

impl<'a, In, Out, I> DoubleEndedIterator for IterMap<'a, In, Out, I>
where
    I: DoubleEndedIterator<Item = In>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(&self.func)
    }
}

impl<'a, In, Out, I> ExactSizeIterator for IterMap<'a, In, Out, I> where
    I: ExactSizeIterator<Item = In>
{
}

impl<'a, In, Out, I> iter::FusedIterator for IterMap<'a, In, Out, I> where
    I: iter::FusedIterator<Item = In>
{
}

/// An iterator returned by [`IterTryMapMatcher`].
#[derive(Debug, Clone)]
pub struct IterTryMap<Out> {
    inner: std::vec::IntoIter<Out>,
}

impl<Out> Iterator for IterTryMap<Out> {
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<Out> DoubleEndedIterator for IterTryMap<Out> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<Out> ExactSizeIterator for IterTryMap<Out> {}

impl<Out> iter::FusedIterator for IterTryMap<Out> {}

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
    pub fn new(func: impl Fn(In) -> Out + 'a) -> Self {
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

    type PosOut = IterMap<'a, In, Out, IntoIter::IntoIter>;
    type NegOut = IterMap<'a, In, Out, IntoIter::IntoIter>;

    type PosFail = Infallible;
    type NegFail = Infallible;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        Ok(MatchOutcome::Success(IterMap {
            inner: actual.into_iter(),
            func: self.func,
        }))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.match_pos(actual)
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
    pub fn new(func: impl Fn(In) -> crate::Result<Out> + 'a) -> Self {
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

    type PosOut = IterTryMap<Out>;
    type NegOut = IterTryMap<Out>;

    type PosFail = Infallible;
    type NegFail = Infallible;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        Ok(MatchOutcome::Success(IterTryMap {
            inner: actual
                .into_iter()
                .map(self.func)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        }))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        self.match_pos(actual)
    }
}
