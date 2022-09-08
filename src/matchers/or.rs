use std::fmt;

use crate::{
    DynMatchNeg, DynMatchPos, Format, Formatter, Matcher, MatchNeg, MatchPos, MatchBase, DynMatchFailure, MatchResult,
    ResultFormat, MatchFailure,
};

#[derive(Debug)]
struct BaseOrAssertion<'a, T> {
    value: T,
    failures: &'a mut Vec<Option<DynMatchFailure>>,
}

impl<'a, T> BaseOrAssertion<'a, T> {
    fn new(value: T, failures: &'a mut Vec<Option<DynMatchFailure>>) -> Self {
        Self { value, failures }
    }

    pub fn to<Out>(self, matcher: &mut impl DynMatchPos<In = T, PosOut = Out>) -> anyhow::Result<()> {
        match matcher.match_pos(self.value) {
            Ok(MatchResult::Success(_)) => {
                self.failures.push(None);
                Ok(())
            }
            Ok(MatchResult::Fail(result)) => {
                self.failures.push(Some(result));
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub fn to_not<Out>(self, matcher: &mut impl DynMatchNeg<In = T, NegOut = Out>) -> anyhow::Result<()> {
        match matcher.match_neg(self.value) {
            Ok(MatchResult::Success(_)) => {
                self.failures.push(None);
                Ok(())
            }
            Ok(MatchResult::Fail(result)) => {
                self.failures.push(Some(result));
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct ByRefOrAssertion<'a, T> {
    value: &'a T,
    failures: &'a mut Vec<Option<DynMatchFailure>>,
}

impl<'a, T> ByRefOrAssertion<'a, T> {
    pub fn to<Out: 'a>(self, matcher: &mut impl DynMatchPos<In = &'a T, PosOut = &'a Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value, self.failures)
            .to(matcher)
            .and(Ok(self))
    }

    pub fn to_not<Out: 'a>(self, matcher: &mut impl DynMatchNeg<In = &'a T, NegOut = &'a Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value, self.failures)
            .to_not(matcher)
            .and(Ok(self))
    }
}

#[derive(Debug)]
pub struct CopiedOrAssertion<'a, T> {
    value: T,
    failures: &'a mut Vec<Option<DynMatchFailure>>,
}

impl<'a, T> CopiedOrAssertion<'a, T>
where
    T: Copy,
{
    pub fn to<Out>(self, matcher: &mut impl DynMatchPos<In = T, PosOut = Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value, self.failures)
            .to(matcher)
            .and(Ok(self))
    }

    pub fn to_not<Out>(self, matcher: &mut impl DynMatchNeg<In = T, NegOut = Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value, self.failures)
            .to_not(matcher)
            .and(Ok(self))
    }
}

#[derive(Debug)]
pub struct ClonedOrAssertion<'a, T> {
    value: T,
    failures: &'a mut Vec<Option<DynMatchFailure>>,
}

impl<'a, T> ClonedOrAssertion<'a, T>
where
    T: Clone,
{
    pub fn to<Out>(self, matcher: &mut impl DynMatchPos<In = T, PosOut = Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value.clone(), self.failures)
            .to(matcher)
            .and(Ok(self))
    }

    pub fn to_not<Out>(self, matcher: &mut impl DynMatchNeg<In = T, NegOut = Out>) -> anyhow::Result<Self> {
        BaseOrAssertion::new(self.value.clone(), self.failures)
            .to_not(matcher)
            .and(Ok(self))
    }
}

#[derive(Debug)]
pub struct OrContext<T> {
    value: T,
    failures: Vec<Option<DynMatchFailure>>,
}

impl<T> OrContext<T> {
    pub fn by_ref(&mut self) -> ByRefOrAssertion<T> {
        ByRefOrAssertion {
            value: &self.value,
            failures: &mut self.failures,
        }
    }
}

impl<T> OrContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CopiedOrAssertion<T> {
        CopiedOrAssertion {
            value: self.value.clone(),
            failures: &mut self.failures,
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
            failures: &mut self.failures,
        }
    }
}

pub struct OrMatcher<T>(Box<dyn Fn(&mut OrContext<T>) -> anyhow::Result<()>>);

impl<T> fmt::Debug for OrMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OrMatcher").finish()
    }
}

impl<T> OrMatcher<T> {
    pub fn new(block: impl Fn(&mut OrContext<T>) -> anyhow::Result<()> + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<T> MatchBase for OrMatcher<T> {
    type In = T;
}

impl<T> MatchPos for OrMatcher<T> {
    type PosOut = T;
    type PosFail = Vec<DynMatchFailure>;

    fn match_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let mut assertion = OrContext {
            value: actual,
            failures: Vec::new(),
        };

        (self.0)(&mut assertion)?;

        if assertion.failures.iter().any(Option::is_none) {
            Ok(MatchResult::Success(assertion.value))
        } else {
            Ok(MatchResult::Fail(
                assertion
                    .failures
                    .into_iter()
                    .filter_map(std::convert::identity)
                    .collect(),
            ))
        }
    }
}

impl<T> MatchNeg for OrMatcher<T> {
    type NegOut = T;
    type NegFail = Vec<Option<DynMatchFailure>>;

    fn match_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        let mut assertion = OrContext {
            value: actual,
            failures: Vec::new(),
        };

        (self.0)(&mut assertion)?;

        if assertion.failures.iter().any(Option::is_none) {
            Ok(MatchResult::Fail(assertion.failures))
        } else {
            Ok(MatchResult::Success(assertion.value))
        }
    }
}

#[derive(Debug)]
pub struct OrFormat(MatchFailure<Vec<DynMatchFailure>, Vec<Option<DynMatchFailure>>>);

impl Format for OrFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl From<MatchFailure<Vec<DynMatchFailure>, Vec<Option<DynMatchFailure>>>> for OrFormat {
    fn from(result: MatchFailure<Vec<DynMatchFailure>, Vec<Option<DynMatchFailure>>>) -> Self {
        Self(result)
    }
}

impl ResultFormat for OrFormat {
    type PosFail = Vec<DynMatchFailure>;
    type NegFail = Vec<Option<DynMatchFailure>>;
}

pub fn or<'a, T>(
    block: impl Fn(&mut OrContext<T>) -> anyhow::Result<()> + 'static,
) -> Matcher<'a, T, T, T>
where
    T: 'a,
{
    Matcher::new::<_, OrFormat>(OrMatcher::new(block))
}
