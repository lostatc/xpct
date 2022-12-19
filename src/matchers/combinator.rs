use std::borrow::Borrow;
use std::fmt;

use crate::core::{DynMatch, FormattedFailure, Match, MatchOutcome};
use crate::{fail, success};

/// When some of the given matchers failed.
pub type SomeFailures = Vec<Option<FormattedFailure>>;

/// How a combinator matcher should match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombinatorMode {
    /// Succeed when any matcher matches.
    Any,

    /// Succeed when all matchers match.
    All,
}

type CombinatorState = crate::Result<SomeFailures>;

pub struct CombinatorAssertion<'a, 'b, T, In> {
    value: &'b T,
    state: &'b mut CombinatorState,
    transform: Box<dyn Fn(&'a T) -> In + 'b>,
    negated: bool,
}

impl<'a, 'b, T, In> fmt::Debug for CombinatorAssertion<'a, 'b, T, In>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CombinatorAssertion")
            .field("value", &self.value)
            .field("state", &self.state)
            .field("negated", &self.negated)
            .finish_non_exhaustive()
    }
}

impl<'a, 'b: 'a, T, In> CombinatorAssertion<'a, 'b, T, In> {
    fn match_pos(self, matcher: impl DynMatch<In = In>) -> Self {
        if let Ok(failures) = self.state {
            match Box::new(matcher).match_pos((self.transform)(self.value)) {
                Ok(MatchOutcome::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchOutcome::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = Err(error);
                }
            }
        };

        self
    }

    fn match_neg(self, matcher: impl DynMatch<In = In>) -> Self {
        if let Ok(failures) = self.state {
            match Box::new(matcher).match_neg((self.transform)(self.value)) {
                Ok(MatchOutcome::Success(_)) => {
                    failures.push(None);
                }
                Ok(MatchOutcome::Fail(result)) => {
                    failures.push(Some(result));
                }
                Err(error) => {
                    *self.state = Err(error);
                }
            }
        };

        self
    }

    pub fn to(self, matcher: impl DynMatch<In = In>) -> Self {
        if self.negated {
            self.match_neg(matcher)
        } else {
            self.match_pos(matcher)
        }
    }

    pub fn to_not(self, matcher: impl DynMatch<In = In>) -> Self {
        if self.negated {
            self.match_pos(matcher)
        } else {
            self.match_neg(matcher)
        }
    }

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct CombinatorContext<T> {
    value: T,
    state: CombinatorState,
    negated: bool,
}

impl<T> CombinatorContext<T> {
    fn new(value: T, negated: bool) -> Self {
        CombinatorContext {
            value,
            state: Ok(Vec::new()),
            negated,
        }
    }

    pub fn borrow<Borrowed: ?Sized>(&mut self) -> CombinatorAssertion<T, &Borrowed>
    where
        T: Borrow<Borrowed>,
    {
        self.map(|value| value.borrow())
    }

    pub fn map<'a, 'b: 'a, In>(
        &'b mut self,
        func: impl Fn(&'a T) -> In + 'b,
    ) -> CombinatorAssertion<'a, 'b, T, In> {
        CombinatorAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(func),
            negated: self.negated,
        }
    }
}

impl<T> CombinatorContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> CombinatorAssertion<T, T> {
        CombinatorAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(|value| *value),
            negated: self.negated,
        }
    }
}

impl<T> CombinatorContext<T>
where
    T: Clone,
{
    pub fn cloned(&mut self) -> CombinatorAssertion<T, T> {
        CombinatorAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(|value| value.clone()),
            negated: self.negated,
        }
    }
}

type BoxCombinatorFunc<'a, T> = Box<dyn FnOnce(&mut CombinatorContext<T>) + 'a>;

pub struct CombinatorMatcher<'a, T> {
    mode: CombinatorMode,
    func: BoxCombinatorFunc<'a, T>,
}

impl<'a, T> fmt::Debug for CombinatorMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CombinatorMatcher")
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

impl<'a, T> CombinatorMatcher<'a, T> {
    pub fn new(mode: CombinatorMode, block: impl FnOnce(&mut CombinatorContext<T>) + 'a) -> Self {
        Self {
            mode,
            func: Box::new(block),
        }
    }
}

impl<'a, T> Match for CombinatorMatcher<'a, T> {
    type In = T;

    type PosOut = T;
    type NegOut = T;

    type PosFail = SomeFailures;
    type NegFail = SomeFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let mut ctx = CombinatorContext::new(actual, false);

        (self.func)(&mut ctx);

        match (ctx.state, self.mode) {
            (Ok(failures), CombinatorMode::Any) => {
                if failures.iter().any(Option::is_none) {
                    success!(ctx.value);
                } else {
                    fail!(failures);
                }
            }
            (Ok(failures), CombinatorMode::All) => {
                if failures.iter().any(Option::is_some) {
                    fail!(failures);
                } else {
                    success!(ctx.value);
                }
            }
            (Err(error), _) => Err(error),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        let mut ctx = CombinatorContext::new(actual, true);

        (self.func)(&mut ctx);

        match (ctx.state, self.mode) {
            (Ok(failures), CombinatorMode::Any) => {
                if failures.iter().any(Option::is_some) {
                    fail!(failures);
                } else {
                    success!(ctx.value);
                }
            }
            (Ok(failures), CombinatorMode::All) => {
                if failures.iter().any(Option::is_none) {
                    success!(ctx.value);
                } else {
                    fail!(failures);
                }
            }
            (Err(error), _) => Err(error),
        }
    }
}
