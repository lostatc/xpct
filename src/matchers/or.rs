use std::fmt;

use crate::{
    DynMatchNeg, DynMatchPos, Format, Formatter, Matcher, MatchNeg, MatchPos, MatchBase, DynMatchFailure, MatchResult,
    ResultFormat, MatchFailure,
};

pub type AllFailures = Vec<DynMatchFailure>;

pub type SomeFailures = Vec<Option<DynMatchFailure>>;

#[derive(Debug)]
enum OrAssertionState {
    Ok(SomeFailures),
    Err(anyhow::Error),
}

impl OrAssertionState {
    fn new() -> Self {
        Self::Ok(Vec::new())
    }
}

#[derive(Debug)]
struct BaseOrAssertion<'a, T> {
    value: T,
    state: &'a mut OrAssertionState,
}

impl<'a, T> BaseOrAssertion<'a, T> {
    fn new(value: T, state: &'a mut OrAssertionState) -> Self {
        Self { value, state }
    }

    fn to<Out>(self, matcher: impl DynMatchPos<In = T, PosOut = Out>) {
        if let OrAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_pos(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = OrAssertionState::Err(error);
                },
            }
        }
    }

    fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) {
        if let OrAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_neg(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = OrAssertionState::Err(error);
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct ByRefOrAssertion<'a, T> {
    value: &'a T,
    state: &'a mut OrAssertionState,
}

impl<'a, T> ByRefOrAssertion<'a, T>
where
    T: 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = &'a T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = &'a T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }
}

#[derive(Debug)]
pub struct CopiedOrAssertion<'a, T> {
    value: T,
    state: &'a mut OrAssertionState,
}

impl<'a, T> CopiedOrAssertion<'a, T>
where
    T: Copy + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value, self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value, self.state);
        assertion.to_not(matcher);
        self
    }
}

#[derive(Debug)]
pub struct ClonedOrAssertion<'a, T> {
    value: T,
    state: &'a mut OrAssertionState,
}

impl<'a, T> ClonedOrAssertion<'a, T>
where
    T: Clone + 'a,
{
    pub fn to(self, matcher: impl DynMatchPos<In = T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value.clone(), self.state);
        assertion.to(matcher);
        self
    }

    pub fn to_not(self, matcher: impl DynMatchNeg<In = T>) -> Self {
        let assertion = BaseOrAssertion::new(self.value.clone(), self.state);
        assertion.to_not(matcher);
        self
    }
}

#[derive(Debug)]
pub struct OrContext<T> {
    value: T,
    state: OrAssertionState,
}

impl<T> OrContext<T> {
    fn new(value: T) -> Self {
        OrContext { value, state: OrAssertionState::new() }
    }
}

impl<T> OrContext<T> {
    pub fn by_ref(&mut self) -> ByRefOrAssertion<T> {
        ByRefOrAssertion {
            value: &self.value,
            state: &mut self.state,
        }
    }
}

impl<T> OrContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CopiedOrAssertion<T> {
        CopiedOrAssertion {
            value: self.value,
            state: &mut self.state,
        }
    }
}

impl<T> OrContext<T>
where
    T: Clone,
{
    pub fn cloned(&mut self) -> ClonedOrAssertion<T> {
        ClonedOrAssertion {
            value: self.value.clone(),
            state: &mut self.state,
        }
    }
}

pub struct OrMatcher<T>(Box<dyn Fn(&mut OrContext<T>)>);

impl<T> fmt::Debug for OrMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OrMatcher").finish()
    }
}

impl<T> OrMatcher<T> {
    pub fn new(block: impl Fn(&mut OrContext<T>) + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<T> MatchBase for OrMatcher<T> {
    type In = T;
}

impl<T> MatchPos for OrMatcher<T> {
    type PosOut = T;
    type PosFail = AllFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut ctx = OrContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            OrAssertionState::Ok(failures) => if failures.iter().any(Option::is_none) {
                Ok(MatchResult::Success(ctx.value))
            } else {
                Ok(MatchResult::Fail(
                    failures
                        .into_iter()
                        .filter_map(std::convert::identity)
                        .collect(),
                ))
            },
            OrAssertionState::Err(error) => Err(error),
        }
    }
}

impl<T> MatchNeg for OrMatcher<T> {
    type NegOut = T;
    type NegFail = SomeFailures;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        let mut ctx = OrContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            OrAssertionState::Ok(failures) => if failures.iter().any(Option::is_none) {
                Ok(MatchResult::Fail(failures))
            } else {
                Ok(MatchResult::Success(ctx.value))
            },
            OrAssertionState::Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct OrFormat(MatchFailure<AllFailures, SomeFailures>);

impl Format for OrFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl ResultFormat for OrFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub fn or<T>(
    block: impl Fn(&mut OrContext<T>) + 'static,
) -> Matcher<T, T, T>
where
    T: 'static,
{
    Matcher::new::<_, OrFormat>(OrMatcher::new(block))
}
