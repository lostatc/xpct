use std::any::type_name;
use std::fmt;

use crate::core::{
    DynMatchFailure, DynMatchNeg, DynMatchPos, MatchBase, MatchError, MatchPos, MatchResult,
};

#[derive(Debug)]
pub struct NoneAssertion<T> {
    value: T,
}

impl<T> NoneAssertion<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> NoneAssertion<T> {
    pub fn to<Out>(
        self,
        matcher: impl DynMatchNeg<In = T, NegOut = Out>,
    ) -> Result<NoneAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Ok(NoneAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchPos<In = T, PosOut = Out>,
    ) -> Result<NoneAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchResult::Success(out)) => Ok(NoneAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }
}

pub struct NoneMatcher<'a, In, Out>(
    Box<dyn FnOnce(NoneAssertion<In>) -> Result<NoneAssertion<Out>, MatchError> + 'a>,
);

impl<'a, In, Out> fmt::Debug for NoneMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NoneMatcher")
            .field(&type_name::<
                Box<dyn FnOnce(NoneAssertion<In>) -> Result<NoneAssertion<Out>, MatchError> + 'a>,
            >())
            .finish()
    }
}

impl<'a, In, Out> NoneMatcher<'a, In, Out> {
    pub fn new(
        block: impl FnOnce(NoneAssertion<In>) -> Result<NoneAssertion<Out>, MatchError> + 'a,
    ) -> Self {
        Self(Box::new(block))
    }
}

impl<'a, In, Out> MatchBase for NoneMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for NoneMatcher<'a, In, Out> {
    type PosOut = Out;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match (self.0)(NoneAssertion::new(actual)) {
            Ok(assertion) => Ok(MatchResult::Success(assertion.value)),
            Err(MatchError::Fail(fail)) => Ok(MatchResult::Fail(fail)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}
