use crate::core::{DynTransformMatch, FormattedFailure, MatchError, MatchOutcome, TransformMatch};
use std::fmt;

use super::IterMap;

/// A type used with [`ChainMatcher`] to compose assertions.
///
/// This type is analogous to [`Assertion`], and has many of the same methods.
///
/// [`Assertion`]: crate::core::Assertion
#[derive(Debug)]
pub struct ChainAssertion<In> {
    value: In,
}

impl<In> ChainAssertion<In> {
    /// Create a new [`ChainAssertion`].
    fn new(value: In) -> Self {
        Self { value }
    }
}

impl<In> ChainAssertion<In> {
    /// Make an assertion with the given `matcher`.
    ///
    /// This does the same thing as [`Assertion::to`].
    ///
    /// [`Assertion::to`]: crate::core::Assertion::to
    pub fn to<Out>(
        self,
        matcher: impl DynTransformMatch<In = In, PosOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    /// Same as [`to`], but negated.
    ///
    /// This does the same thing as [`Assertion::to_not`].
    ///
    /// This tests that the given matcher does *not* succeed.
    ///
    /// [`to`]: crate::matchers::ChainAssertion::to
    /// [`Assertion::to_not`]: crate::core::Assertion::to_not
    pub fn to_not<Out>(
        self,
        matcher: impl DynTransformMatch<In = In, NegOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    /// Infallibly map the input value by applying a function to it.
    ///
    /// This does the same thing as [`Assertion::map`].
    ///
    /// [`Assertion::map`]: crate::core::Assertion::map
    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> ChainAssertion<Out> {
        ChainAssertion::new(func(self.value))
    }

    /// Fallibly map the input value by applying a function to it.
    ///
    /// This does the same thing as [`Assertion::try_map`].
    ///
    /// [`Assertion::try_map`]: crate::core::Assertion::try_map
    pub fn try_map<Out>(
        self,
        func: impl FnOnce(In) -> crate::Result<Out>,
    ) -> crate::Result<ChainAssertion<Out>> {
        Ok(ChainAssertion::new(func(self.value)?))
    }

    /// Infallibly convert the input value via [`From`]/[`Into`].
    ///
    /// This does the same thing as [`Assertion::into`].
    ///
    /// [`Assertion::into`]: crate::core::Assertion::into
    pub fn into<Out>(self) -> ChainAssertion<Out>
    where
        Out: From<In>,
    {
        ChainAssertion::new(self.value.into())
    }

    /// Fallibly convert the input value via [`TryFrom`]/[`TryInto`].
    ///
    /// This does the same thing as [`Assertion::try_into`].
    ///
    /// [`Assertion::try_into`]: crate::core::Assertion::try_into
    pub fn try_into<Out>(self) -> crate::Result<ChainAssertion<Out>>
    where
        Out: TryFrom<In>,
        <Out as TryFrom<In>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Ok(ChainAssertion::new(self.value.try_into()?))
    }
}

impl<In> ChainAssertion<In>
where
    In: IntoIterator,
{
    /// Infallibly map each value of an iterator by applying a function to it.
    ///
    /// This does the same thing as [`Assertion::iter_map`].
    ///
    /// [`Assertion::iter_map`]: crate::core::Assertion::iter_map
    pub fn iter_map<'a, Out>(
        self,
        func: impl Fn(In::Item) -> Out + 'a,
    ) -> ChainAssertion<IterMap<'a, In::Item, Out, In::IntoIter>> {
        ChainAssertion::new(IterMap::new(self.value.into_iter(), Box::new(func)))
    }

    /// Fallibly map each value of an iterator by applying a function to it.
    ///
    /// This does the same thing as [`Assertion::iter_try_map`].
    ///
    /// [`Assertion::iter_try_map`]: crate::core::Assertion::iter_try_map
    pub fn iter_try_map<'a, Out>(
        self,
        func: impl Fn(In::Item) -> crate::Result<Out> + 'a,
    ) -> crate::Result<ChainAssertion<Vec<Out>>> {
        let mapped_values = self
            .value
            .into_iter()
            .map(func)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ChainAssertion::new(mapped_values))
    }
}

type BoxChainFunc<'a, In, Out> =
    Box<dyn FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a>;

/// The matcher for [`all`].
///
/// [`all`]: crate::all
pub struct ChainMatcher<'a, In, Out> {
    func: BoxChainFunc<'a, In, Out>,
}

impl<'a, In, Out> fmt::Debug for ChainMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out> ChainMatcher<'a, In, Out> {
    /// Create a new [`ChainMatcher`].
    pub fn new(
        block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
    ) -> Self {
        Self {
            func: Box::new(block),
        }
    }
}

impl<'a, In, Out> TransformMatch for ChainMatcher<'a, In, Out> {
    type In = In;

    type PosOut = Out;
    type NegOut = ();

    type PosFail = FormattedFailure;
    type NegFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match (self.func)(ChainAssertion::new(actual)) {
            Ok(assertion) => Ok(MatchOutcome::Success(assertion.value)),
            Err(MatchError::Fail(fail)) => Ok(MatchOutcome::Fail(fail)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match (self.func)(ChainAssertion::new(actual)) {
            Ok(_) => Ok(MatchOutcome::Fail(())),
            Err(MatchError::Fail(_)) => Ok(MatchOutcome::Success(())),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}
