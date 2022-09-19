use crate::core::{
    DynMatchNeg, DynMatchPos, FormattedFailure, MatchBase, MatchError, MatchPos, MatchResult,
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
            Ok(MatchResult::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = In, NegOut = Out>,
    ) -> Result<ChainAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Ok(ChainAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> ChainAssertion<Out> {
        ChainAssertion::new(func(self.value))
    }

    pub fn map_result<Out>(
        self,
        func: impl FnOnce(In) -> anyhow::Result<Out>,
    ) -> anyhow::Result<ChainAssertion<Out>> {
        Ok(ChainAssertion::new(func(self.value)?))
    }

    pub fn into<Out>(self) -> ChainAssertion<Out>
    where
        Out: From<In>,
    {
        ChainAssertion::new(self.value.into())
    }

    pub fn try_into<Out>(self) -> anyhow::Result<ChainAssertion<Out>>
    where
        Out: TryFrom<In>,
        <Out as TryFrom<In>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Ok(ChainAssertion::new(self.value.try_into()?))
    }
}

pub struct ChainMatcher<'a, In, Out>(
    Box<dyn FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a>,
);

impl<'a, In, Out> fmt::Debug for ChainMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ChainMatcher")
            .field(&type_name::<
                Box<dyn FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a>,
            >())
            .finish()
    }
}

impl<'a, In, Out> ChainMatcher<'a, In, Out> {
    pub fn new(
        block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
    ) -> Self {
        Self(Box::new(block))
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
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match (self.0)(ChainAssertion::new(actual)) {
            Ok(assertion) => success!(assertion.value),
            Err(MatchError::Fail(fail)) => fail!(fail),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}
