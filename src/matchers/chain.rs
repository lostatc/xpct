use crate::core::{DynMatch, FormattedFailure, Match, MatchError, MatchOutcome};
use crate::{fail, success};
use std::fmt;

/// A type used with [`ChainMatcher`] to compose assertions.
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
    pub fn to<Out>(
        self,
        matcher: impl DynMatch<In = In, PosOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    /// Same as [`to`], but negated.
    ///
    /// This tests that the given matcher does *not* succeed.
    ///
    /// [`to`]: crate::matchers::ChainAssertion::to
    pub fn to_not<Out>(
        self,
        matcher: impl DynMatch<In = In, NegOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    /// Infallibly map the input value to an output value, possibly of a different type.
    ///
    /// This does the same thing as [`Assertion::map`].
    ///
    /// [`Assertion::map`]: crate::core::Assertion::map
    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> ChainAssertion<Out> {
        ChainAssertion::new(func(self.value))
    }

    /// Fallibly map the input value to an output value, possibly of a different type.
    ///
    /// This does the same thing as [`Assertion::try_map`].
    ///
    /// [`Assertion::try_map`]: crate::core::Assertion::map
    pub fn try_map<Out>(
        self,
        func: impl FnOnce(In) -> crate::Result<Out>,
    ) -> crate::Result<ChainAssertion<Out>> {
        Ok(ChainAssertion::new(func(self.value)?))
    }

    /// Convert the input value via [`Into`].
    ///
    /// [`expect!`]: crate::expect
    pub fn into<Out>(self) -> ChainAssertion<Out>
    where
        Out: From<In>,
    {
        ChainAssertion::new(self.value.into())
    }

    /// Same as [`into`], but with [`TryInto`].
    ///
    /// [`into`]: crate::matchers::ChainAssertion::into
    pub fn try_into<Out>(self) -> crate::Result<ChainAssertion<Out>>
    where
        Out: TryFrom<In>,
        <Out as TryFrom<In>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Ok(ChainAssertion::new(self.value.try_into()?))
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

impl<'a, In, Out> Match for ChainMatcher<'a, In, Out> {
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
            Ok(assertion) => success!(assertion.value),
            Err(MatchError::Fail(fail)) => fail!(fail),
            Err(MatchError::Err(error)) => Err(error),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match (self.func)(ChainAssertion::new(actual)) {
            Ok(_) => fail!(()),
            Err(MatchError::Fail(_)) => success!(()),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}
