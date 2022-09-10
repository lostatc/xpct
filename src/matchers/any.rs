use std::fmt;

use crate::fmt::indexed_list;
use crate::{
    DynMatchFailure, DynMatchNeg, DynMatchPos, Format, Formatter, MatchBase, MatchFailure,
    MatchNeg, MatchPos, MatchResult, Matcher, ResultFormat,
};

#[derive(Debug)]
pub struct AllFailures(pub Vec<DynMatchFailure>);

impl Format for AllFailures {
    fn fmt(&self, f: &mut Formatter) {
        indexed_list(f, self.0.iter().map(AsRef::as_ref));
    }
}

#[derive(Debug)]
pub struct SomeFailures(pub Vec<Option<DynMatchFailure>>);

impl Format for SomeFailures {
    fn fmt(&self, f: &mut Formatter) {
        indexed_list(
            f,
            self.0.iter().map(|maybe_fail| match maybe_fail {
                Some(fail) => fail.as_ref(),
                None => "<matched>",
            }),
        );
    }
}

impl SomeFailures {
    pub fn has_any(&self) -> bool {
        self.0.iter().any(Option::is_none)
    }

    pub fn has_all(&self) -> bool {
        self.0.iter().all(Option::is_none)
    }

    pub fn count(&self) -> usize {
        self.0.iter().filter(|item| item.is_none()).count()
    }

    pub fn filter_into(self) -> AllFailures {
        AllFailures(
            self.0
                .into_iter()
                .filter_map(std::convert::identity)
                .collect(),
        )
    }
}

#[derive(Debug)]
enum AnyAssertionState {
    Ok(SomeFailures),
    Err(anyhow::Error),
}

impl AnyAssertionState {
    fn new() -> Self {
        Self::Ok(SomeFailures(Vec::new()))
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
                    failures.0.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.0.push(Some(result));
                }
                Err(error) => {
                    *self.state = AnyAssertionState::Err(error);
                }
            }
        }
    }

    fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) {
        if let AnyAssertionState::Ok(failures) = self.state {
            match Box::new(matcher).match_neg(self.value) {
                Ok(MatchResult::Success(_)) => {
                    failures.0.push(None);
                }
                Ok(MatchResult::Fail(result)) => {
                    failures.0.push(Some(result));
                }
                Err(error) => {
                    *self.state = AnyAssertionState::Err(error);
                }
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
        AnyContext {
            value,
            state: AnyAssertionState::new(),
        }
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
                if failures.has_any() {
                    Ok(MatchResult::Success(ctx.value))
                } else {
                    Ok(MatchResult::Fail(failures.filter_into()))
                }
            }
            AnyAssertionState::Err(error) => Err(error),
        }
    }
}

impl<'a, T> MatchNeg for AnyMatcher<'a, T> {
    type NegOut = T;
    type NegFail = SomeFailures;

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        let mut ctx = AnyContext::new(actual);

        (self.0)(&mut ctx);

        match ctx.state {
            AnyAssertionState::Ok(failures) => {
                if failures.has_any() {
                    Ok(MatchResult::Fail(failures))
                } else {
                    Ok(MatchResult::Success(ctx.value))
                }
            }
            AnyAssertionState::Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct AnyFormat(MatchFailure<AllFailures, SomeFailures>);

impl Format for AnyFormat {
    fn fmt(&self, f: &mut Formatter) {
        match &self.0 {
            MatchFailure::Pos(failures) => {
                f.write_str("Expected at least one of these to match, but none did:");
                f.writeln();
                f.set_indent(2);
                f.write_fmt(failures);
            }
            MatchFailure::Neg(failures) => {
                f.write_str(&format!(
                    "Expected none of these to match, but {} did:",
                    failures.count(),
                ));
                f.writeln();
                f.set_indent(2);
                f.write_fmt(failures);
            }
        }
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub fn any<'a, T>(block: impl Fn(&mut AnyContext<T>) + 'a) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::new::<AnyFormat, _>(AnyMatcher::new(block))
}
