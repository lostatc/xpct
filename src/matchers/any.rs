use std::fmt;

use crate::{
    DynMatchNeg, DynMatchPos, Format, Formatter, Matcher, MatchNeg, MatchPos, MatchBase, DynMatchFailure, MatchResult,
    ResultFormat, MatchFailure,
};

pub type AllFailures = Vec<DynMatchFailure>;

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

    fn to<Out>(self, matcher: impl DynMatchPos<In = T, PosOut = Out>) {
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
                },
            }
        }
    }

    fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) {
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
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct ByRefAnyAssertion<'a, T> {
    value: &'a T,
    state: &'a mut AnyAssertionState,
}

impl<'a, T> ByRefAnyAssertion<'a, T>
where
    T: 'a,
{
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
        AnyContext { value, state: AnyAssertionState::new() }
    }
}

impl<T> AnyContext<T> {
    pub fn by_ref(&mut self) -> ByRefAnyAssertion<T> {
        ByRefAnyAssertion {
            value: &self.value,
            state: &mut self.state,
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

pub struct AnyMatcher<T>(Box<dyn Fn(&mut AnyContext<T>)>);

impl<T> fmt::Debug for AnyMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AnyMatcher").finish()
    }
}

impl<T> AnyMatcher<T> {
    pub fn new(block: impl Fn(&mut AnyContext<T>) + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<T> MatchBase for AnyMatcher<T> {
    type In = T;
}

impl<T> MatchPos for AnyMatcher<T> {
    type PosOut = T;
    type PosFail = AllFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut ctx = AnyContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            AnyAssertionState::Ok(failures) => if failures.iter().any(Option::is_none) {
                Ok(MatchResult::Success(ctx.value))
            } else {
                Ok(MatchResult::Fail(
                    failures
                        .into_iter()
                        .filter_map(std::convert::identity)
                        .collect(),
                ))
            },
            AnyAssertionState::Err(error) => Err(error),
        }
    }
}

impl<T> MatchNeg for AnyMatcher<T> {
    type NegOut = T;
    type NegFail = SomeFailures;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        let mut ctx = AnyContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            AnyAssertionState::Ok(failures) => if failures.iter().any(Option::is_none) {
                Ok(MatchResult::Fail(failures))
            } else {
                Ok(MatchResult::Success(ctx.value))
            },
            AnyAssertionState::Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct AnyFormat(MatchFailure<AllFailures, SomeFailures>);

impl Format for AnyFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub fn any<T>(
    block: impl Fn(&mut AnyContext<T>) + 'static,
) -> Matcher<T, T, T>
where
    T: 'static,
{
    Matcher::new::<_, AnyFormat>(AnyMatcher::new(block))
}
