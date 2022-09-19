use std::any::type_name;
use std::borrow::Borrow;
use std::fmt;

use crate::core::{DynMatchFailure, DynMatchNeg, DynMatchPos, MatchBase, MatchPos, MatchResult};
use crate::{fail, success};

/// When all the given matchers failed.
pub type AllFailures = Vec<DynMatchFailure>;

/// When some of the given matchers failed.
pub type SomeFailures = Vec<Option<DynMatchFailure>>;

#[derive(Debug)]
enum AnyAssertionState {
    Ok(SomeFailures),
    Err(anyhow::Error),
}

impl AnyAssertionState {
    fn new() -> Self {
        Self::Ok(Vec::new())
    }
}

#[derive(Debug)]
struct BaseAnyAssertion<'a, T> {
    value: T,
    state: &'a mut AnyAssertionState,
}

impl<'a, T> BaseAnyAssertion<'a, T> {
    fn new(value: T, state: &'a mut AnyAssertionState) -> Self {
        Self { value, state }
    }

    fn to(self, matcher: impl DynMatchPos<In = T>) {
        if let AnyAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_pos(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = AnyAssertionState::Err(error);
                }
            }
        }
    }

    fn to_not(self, matcher: impl DynMatchNeg<In = T>) {
        if let AnyAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_neg(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = AnyAssertionState::Err(error);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BorrowedAnyAssertion<'a, T: ?Sized> {
    value: &'a T,
    state: &'a mut AnyAssertionState,
}

impl<'a, T: ?Sized> BorrowedAnyAssertion<'a, T> {
    pub fn to(self, matcher: impl DynMatchPos<In = &'a T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = &'a T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

pub struct MappedAnyAssertion<'a, 'b, T, In> {
    value: &'b T,
    state: &'b mut AnyAssertionState,
    transform: Box<dyn Fn(&'a T) -> In + 'b>,
}

impl<'a, 'b, T, In> fmt::Debug for MappedAnyAssertion<'a, 'b, T, In>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MappedAnyAssertion")
            .field("value", &self.value)
            .field("state", &self.state)
            .field("transform", &type_name::<Box<dyn Fn(&'a T) -> In + 'b>>())
            .finish()
    }
}

impl<'a, 'b: 'a, T, In> MappedAnyAssertion<'a, 'b, T, In> {
    pub fn to(self, matcher: impl DynMatchPos<In = In>) -> Self {
        let assertion = BaseAnyAssertion::new((&self.transform)(&self.value), self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = In>) -> Self {
        let assertion = BaseAnyAssertion::new((&self.transform)(&self.value), self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct CopiedAnyAssertion<'a, T> {
    value: T,
    state: &'a mut AnyAssertionState,
}

impl<'a, T> CopiedAnyAssertion<'a, T>
where
    T: Copy + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct ClonedAnyAssertion<'a, T> {
    value: T,
    state: &'a mut AnyAssertionState,
}

impl<'a, T> ClonedAnyAssertion<'a, T>
where
    T: Clone + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value.clone(), self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseAnyAssertion::new(self.value.clone(), self.state);
        assertion.to_not(matcher);
        self
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct AnyContext<T> {
    value: T,
    state: AnyAssertionState,
}

impl<T> AnyContext<T> {
    fn new(value: T) -> Self {
        AnyContext {
            value,
            state: AnyAssertionState::new(),
        }
    }

    pub fn borrow<'a, Borrowed: ?Sized>(&mut self) -> BorrowedAnyAssertion<Borrowed>
    where
        T: Borrow<Borrowed>,
    {
        BorrowedAnyAssertion {
            value: self.value.borrow(),
            state: &mut self.state,
        }
    }

    pub fn map<'a, 'b: 'a, In>(
        &'b mut self,
        func: impl Fn(&'a T) -> In + 'b,
    ) -> MappedAnyAssertion<'a, 'b, T, In> {
        MappedAnyAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(func),
        }
    }
}

impl<T> AnyContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CopiedAnyAssertion<T> {
        CopiedAnyAssertion {
            value: self.value,
            state: &mut self.state,
        }
    }
}

impl<T> AnyContext<T>
where
    T: Clone,
{
    pub fn cloned(&mut self) -> ClonedAnyAssertion<T> {
        ClonedAnyAssertion {
            value: self.value.clone(),
            state: &mut self.state,
        }
    }
}

pub struct AnyMatcher<'a, T>(Box<dyn FnOnce(&mut AnyContext<T>) + 'a>);

impl<'a, T> fmt::Debug for AnyMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AnyMatcher").finish()
    }
}

impl<'a, T> AnyMatcher<'a, T> {
    pub fn new(block: impl FnOnce(&mut AnyContext<T>) + 'a) -> Self {
        Self(Box::new(block))
    }
}

impl<'a, T> MatchBase for AnyMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for AnyMatcher<'a, T> {
    type PosOut = T;
    type PosFail = AllFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut ctx = AnyContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            AnyAssertionState::Ok(failures) => {
                if failures.iter().any(Option::is_none) {
                    success!(ctx.value);
                } else {
                    fail!(failures
                        .into_iter()
                        .filter_map(std::convert::identity)
                        .collect::<Vec<_>>());
                }
            }
            AnyAssertionState::Err(error) => Err(error),
        }
    }
}
