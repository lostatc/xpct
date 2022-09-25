use std::any::type_name;
use std::borrow::Borrow;
use std::fmt;

use crate::core::{DynMatchNeg, DynMatchPos, FormattedFailure, MatchBase, MatchOutcome, MatchPos};
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
}

impl<'a, 'b, T, In> fmt::Debug for CombinatorAssertion<'a, 'b, T, In>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CombinatorAssertion")
            .field("value", &self.value)
            .field("state", &self.state)
            .field("transform", &type_name::<Box<dyn Fn(&'a T) -> In + 'b>>())
            .finish()
    }
}

impl<'a, 'b: 'a, T, In> CombinatorAssertion<'a, 'b, T, In> {
    pub fn to(self, matcher: impl DynMatchPos<In = In>) -> Self {
        if let Ok(failures) = self.state {
            match Box::new(matcher).match_pos((&self.transform)(self.value)) {
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

    pub fn to_not(self, matcher: impl DynMatchNeg<In = In>) -> Self {
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

    pub fn done(self) {}
}

#[derive(Debug)]
pub struct CombinatorContext<T> {
    value: T,
    state: CombinatorState,
}

impl<T> CombinatorContext<T> {
    fn new(value: T) -> Self {
        CombinatorContext {
            value,
            state: Ok(Vec::new()),
        }
    }

    pub fn borrow<'a, Borrowed: ?Sized>(&'a mut self) -> CombinatorAssertion<'a, 'a, T, &Borrowed>
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
        }
    }
}

impl<T> CombinatorContext<T>
where
    T: Copy,
{
    pub fn copied<'a>(&'a mut self) -> CombinatorAssertion<'a, 'a, T, T> {
        CombinatorAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(|value| *value),
        }
    }
}

impl<T> CombinatorContext<T>
where
    T: Clone,
{
    pub fn cloned<'a>(&'a mut self) -> CombinatorAssertion<'a, 'a, T, T> {
        CombinatorAssertion {
            value: &self.value,
            state: &mut self.state,
            transform: Box::new(|value| value.clone()),
        }
    }
}

pub struct CombinatorMatcher<'a, T> {
    mode: CombinatorMode,
    func: Box<dyn FnOnce(&mut CombinatorContext<T>) + 'a>,
}

impl<'a, T> fmt::Debug for CombinatorMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CombinatorMatcher").finish()
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

impl<'a, T> MatchBase for CombinatorMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for CombinatorMatcher<'a, T> {
    type PosOut = T;
    type PosFail = SomeFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let mut ctx = CombinatorContext::new(actual);

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
}
