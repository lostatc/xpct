use crate::{
    DynMatchFailure, DynMatchNeg, DynMatchPos, MatchBase, MatchError, MatchFailure, MatchNeg,
    MatchPos, MatchResult, Matcher, ResultFormat,
};
use std::fmt;

#[derive(Debug)]
struct BaseEachAssertion<T> {
    value: T,
}

impl<T> BaseEachAssertion<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn to<Out>(self, matcher: impl DynMatchPos<In = T, PosOut = Out>) -> Result<(), MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchResult::Success(_)) => Ok(()),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = T, NegOut = Out>,
    ) -> Result<(), MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(_)) => Ok(()),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }
}

#[derive(Debug)]
pub struct ByRefEachAssertion<'a, T> {
    value: &'a T,
}

impl<'a, T> ByRefEachAssertion<'a, T>
where
    T: 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = &'a T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value).to(matcher)?;
        Ok(self)
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = &'a T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value).to_not(matcher)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct CopiedEachAssertion<T> {
    value: T,
}

impl<T> CopiedEachAssertion<T>
where
    T: Copy,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value).to(matcher)?;
        Ok(self)
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value).to_not(matcher)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct ClonedEachAssertion<T> {
    value: T,
}

impl<T> ClonedEachAssertion<T>
where
    T: Clone,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value.clone()).to(matcher)?;
        Ok(self)
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Result<Self, MatchError> {
        BaseEachAssertion::new(self.value.clone()).to_not(matcher)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct EachContext<T> {
    value: T,
}

impl<T> EachContext<T> {
    fn new(value: T) -> Self {
        EachContext { value }
    }
}

impl<T> EachContext<T> {
    pub fn by_ref(&mut self) -> ByRefEachAssertion<T> {
        ByRefEachAssertion { value: &self.value }
    }
}

impl<T> EachContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CopiedEachAssertion<T> {
        CopiedEachAssertion { value: self.value }
    }
}

impl<T> EachContext<T>
where
    T: Clone,
{
    pub fn cloned(&mut self) -> ClonedEachAssertion<T> {
        ClonedEachAssertion {
            value: self.value.clone(),
        }
    }
}

pub struct EachMatcher<'a, T>(Box<dyn FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'a>);

impl<'a, T> fmt::Debug for EachMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EachMatcher").finish()
    }
}

impl<'a, T> EachMatcher<'a, T> {
    pub fn new(block: impl FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'a) -> Self {
        Self(Box::new(block))
    }
}

impl<'a, T> MatchBase for EachMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for EachMatcher<'a, T> {
    type PosOut = T;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut ctx = EachContext::new(actual);

        match (self.0)(&mut ctx) {
            Ok(_) => Ok(MatchResult::Success(ctx.value)),
            Err(MatchError::Fail(fail)) => Ok(MatchResult::Fail(fail)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}

impl<'a, T> MatchNeg for EachMatcher<'a, T> {
    type NegOut = T;
    type NegFail = ();

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        let mut ctx = EachContext::new(actual);

        match (self.0)(&mut ctx) {
            Ok(_) => Ok(MatchResult::Fail(())),
            Err(MatchError::Fail(_)) => Ok(MatchResult::Success(ctx.value)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct EachFormat(MatchFailure<DynMatchFailure, ()>);

impl fmt::Display for EachFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl ResultFormat for EachFormat {
    type Pos = DynMatchFailure;
    type Neg = ();

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub fn each<'a, T>(
    block: impl FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'a,
) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::new::<EachFormat, _>(EachMatcher::new(block))
}
