use crate::core::{
    DynMatchFailure, DynMatchNeg, DynMatchPos, MatchBase, MatchError, MatchPos, MatchResult,
};
use std::any::type_name;
use std::fmt;

#[derive(Debug)]
pub struct AllAssertion<T> {
    value: T,
}

impl<T> AllAssertion<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> AllAssertion<T> {
    pub fn to<Out>(
        self,
        matcher: impl DynMatchPos<In = T, PosOut = Out>,
    ) -> Result<AllAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchResult::Success(out)) => Ok(AllAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = T, NegOut = Out>,
    ) -> Result<AllAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Ok(AllAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }
}

pub struct AllMatcher<'a, In, Out>(
    Box<dyn FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a>,
);

impl<'a, In, Out> fmt::Debug for AllMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AllMatcher")
            .field(&type_name::<
                Box<dyn FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a>,
            >())
            .finish()
    }
}

impl<'a, In, Out> AllMatcher<'a, In, Out> {
    pub fn new(
        block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a,
    ) -> Self {
        Self(Box::new(block))
    }
}

impl<'a, In, Out> MatchBase for AllMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for AllMatcher<'a, In, Out> {
    type PosOut = Out;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match (self.0)(AllAssertion::new(actual)) {
            Ok(assertion) => Ok(MatchResult::Success(assertion.value)),
            Err(MatchError::Fail(fail)) => Ok(MatchResult::Fail(fail)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}

#[cfg(feature = "fmt")]
use crate::core::PosMatcher;

#[cfg(feature = "fmt")]
pub fn all<'a, In, Out>(
    block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    use super::format::AllFormat;

    PosMatcher::new(AllMatcher::new(block), AllFormat)
}
