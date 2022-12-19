use crate::core::{
    DynMatchNeg, DynMatchPos, FormattedFailure, MatchBase, MatchError, MatchNeg, MatchOutcome,
    MatchPos,
};
use crate::{fail, success};
use std::any::type_name;
use std::fmt;

#[derive(Debug)]
pub struct ChainAssertion<In> {
    value: In,
}

impl<In> ChainAssertion<In> {
    fn new(value: In) -> Self {
        Self { value }
    }
}

impl<In> ChainAssertion<In> {
    pub fn to<Out>(
        self,
        matcher: impl DynMatchPos<In = In, PosOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = In, NegOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchOutcome::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchOutcome::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> ChainAssertion<Out> {
        ChainAssertion::new(func(self.value))
    }

    pub fn try_map<Out>(
        self,
        func: impl FnOnce(In) -> crate::Result<Out>,
    ) -> crate::Result<ChainAssertion<Out>> {
        Ok(ChainAssertion::new(func(self.value)?))
    }

    pub fn into<Out>(self) -> ChainAssertion<Out>
    where
        Out: From<In>,
    {
        ChainAssertion::new(self.value.into())
    }

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

pub struct ChainMatcher<'a, In, Out> {
    func: BoxChainFunc<'a, In, Out>,
}

impl<'a, In, Out> fmt::Debug for ChainMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainMatcher").finish_non_exhaustive()
    }
}

impl<'a, In, Out> ChainMatcher<'a, In, Out> {
    pub fn new(
        block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
    ) -> Self {
        Self {
            func: Box::new(block),
        }
    }
}

impl<'a, In, Out> MatchBase for ChainMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for ChainMatcher<'a, In, Out> {
    type PosOut = Out;
    type PosFail = FormattedFailure;

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
}

impl<'a, In, Out> MatchNeg for ChainMatcher<'a, In, Out> {
    type NegOut = ();
    type NegFail = ();

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
