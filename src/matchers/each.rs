use std::any::type_name;
use std::fmt;

use crate::core::{DynMatchNeg, DynMatchPos, MatchBase, MatchPos, MatchResult};

use super::SomeFailures;

#[derive(Debug)]
enum EachAssertionState {
    Ok(SomeFailures),
    Err(anyhow::Error),
}

impl EachAssertionState {
    fn new() -> Self {
        Self::Ok(Vec::new())
    }
}

#[derive(Debug)]
struct BaseEachAssertion<'a, T> {
    value: T,
    state: &'a mut EachAssertionState,
}

impl<'a, T> BaseEachAssertion<'a, T> {
    fn new(value: T, state: &'a mut EachAssertionState) -> Self {
        Self { value, state }
    }

    fn to<Out>(self, matcher: impl DynMatchPos<In = T, PosOut = Out>) {
        if let EachAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_pos(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = EachAssertionState::Err(error);
                }
            }
        }
    }

    fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) {
        if let EachAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_neg(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = EachAssertionState::Err(error);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ByRefEachAssertion<'a, T> {
    value: &'a T,
    state: &'a mut EachAssertionState,
}

impl<'a, T> ByRefEachAssertion<'a, T>
where
    T: 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = &'a T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = &'a T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct CopiedEachAssertion<'a, T> {
    value: T,
    state: &'a mut EachAssertionState,
}

impl<'a, T> CopiedEachAssertion<'a, T>
where
    T: Copy + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct ClonedEachAssertion<'a, T> {
    value: T,
    state: &'a mut EachAssertionState,
}

impl<'a, T> ClonedEachAssertion<'a, T>
where
    T: Clone + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value.clone(), self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseEachAssertion::new(self.value.clone(), self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct EachContext<T> {
    value: T,
    state: EachAssertionState,
}

impl<T> EachContext<T> {
    fn new(value: T) -> Self {
        EachContext {
            value,
            state: EachAssertionState::new(),
        }
    }
}

impl<T> EachContext<T> {
    pub fn by_ref(&mut self) -> ByRefEachAssertion<T> {
        ByRefEachAssertion {
            value: &self.value,
            state: &mut self.state,
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
            state: &mut self.state,
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
            state: &mut self.state,
        }
    }
}

pub struct EachMatcher<'a, T>(Box<dyn FnOnce(&mut EachContext<T>) + 'a>);

impl<'a, T> fmt::Debug for EachMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EachMatcher")
            .field(&type_name::<Box<dyn FnOnce(&mut EachContext<T>) + 'a>>())
            .finish()
    }
}

impl<'a, T> EachMatcher<'a, T> {
    pub fn new(block: impl FnOnce(&mut EachContext<T>) + 'a) -> Self {
        Self(Box::new(block))
    }
}

impl<'a, T> MatchBase for EachMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for EachMatcher<'a, T> {
    type PosOut = T;
    type PosFail = SomeFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut ctx = EachContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            EachAssertionState::Ok(failures) => {
                if failures.iter().any(Option::is_none) {
                    Ok(MatchResult::Fail(failures))
                } else {
                    Ok(MatchResult::Success(ctx.value))
                }
            }
            EachAssertionState::Err(error) => Err(error),
        }
    }
}
