use std::fmt;
use crate::{DynMatchPos, MatchResult, MatchError, DynMatchNeg, MatchBase, MatchPos, DynMatchFailure, MatchNeg, Format, Formatter, MatchFailure, ResultFormat, Matcher};

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

    fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) -> Result<(), MatchError> {
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
        ByRefEachAssertion {
            value: &self.value,
        }
    }
}

impl<T> EachContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CopiedEachAssertion<T> {
        CopiedEachAssertion {
            value: self.value,
        }
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

pub struct EachMatcher<T>(Box<dyn FnOnce(&mut EachContext<T>) -> Result<(), MatchError>>);

impl<T> fmt::Debug for EachMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EachMatcher").finish()
    }
}

impl<T> EachMatcher<T> {
    pub fn new(block: impl FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<T> MatchBase for EachMatcher<T> {
    type In = T;
}

impl<T> MatchPos for EachMatcher<T> {
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

impl<T> MatchNeg for EachMatcher<T> {
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

impl Format for EachFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
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

pub fn each<T>(
    block: impl FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'static,
) -> Matcher<T, T>
where
    T: 'static,
{
    Matcher::new::<EachFormat, _>(EachMatcher::new(block))
}
