use std::borrow::Borrow;
use std::fmt;

use crate::core::{DynTransformMatch, MatchOutcome, TransformMatch};
use crate::matchers::SomeFailures;

/// How a combinator matcher should match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombinatorMode {
    /// Succeed when any matcher succeeds.
    Any,

    /// Succeed when all matchers succeed.
    All,
}

type CombinatorState = crate::Result<SomeFailures>;

/// A type used with [`CombinatorMatcher`] to compose assertions.
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
    fn match_pos(self, matcher: impl DynTransformMatch<In = In>) -> Self {
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

    fn match_neg(self, matcher: impl DynTransformMatch<In = In>) -> Self {
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

    /// Make an assertion with the given `matcher`.
    pub fn to(self, matcher: impl DynTransformMatch<In = In>) -> Self {
        if self.negated {
            self.match_neg(matcher)
        } else {
            self.match_pos(matcher)
        }
    }

    /// Same as [`to`], but negated.
    ///
    /// This does the same thing as [`Assertion::to_not`].
    ///
    /// This tests that the given matcher does *not* succeed.
    ///
    /// [`to`]: crate::core::Assertion::to
    /// [`Assertion::to_not`]: crate::core::Assertion::to_not
    pub fn to_not(self, matcher: impl DynTransformMatch<In = In>) -> Self {
        if self.negated {
            self.match_pos(matcher)
        } else {
            self.match_neg(matcher)
        }
    }

    /// Consumes `self` and returns `()`.
    ///
    /// This method is a no-op; it just exists for ergonomics. See the example below.
    ///
    /// # Examples
    ///
    /// You can use this method to write the closure as an expression instead of a statement.
    ///
    /// ```
    /// use xpct::{expect, each, be_lt, be_gt};
    ///
    /// expect!(20.0).to(each(|ctx| {
    ///     ctx.copied()
    ///         .to(be_lt(130.0))
    ///         .to(be_gt(0.40));
    /// }));
    ///
    /// expect!(20.0).to(each(|ctx| ctx
    ///     .copied()
    ///     .to(be_lt(130.0))
    ///     .to(be_gt(0.40))
    ///     .done()
    /// ));
    /// ```
    pub fn done(self) {}
}

/// A type used with [`CombinatorMatcher`] to borrow, clone, or copy the owned value.
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

    /// Borrow the owned value before making assertions on it.
    pub fn borrow<Borrowed: ?Sized>(&mut self) -> CombinatorAssertion<T, &Borrowed>
    where
        T: Borrow<Borrowed>,
    {
        self.map(|value| value.borrow())
    }

    /// Map the owned value before making assertions on it.
    ///
    /// This is useful if borrowing, cloning, or copying alone aren't flexible enough. One case
    /// where this may be useful is calling methods like [`Option::as_deref`].
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
    /// Copy the owned value before making assertions on it.
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
    /// Clone the owned value before making assertions on it.
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

/// The matcher for [`any`] and [`each`].
///
/// [`any`]: crate::any
/// [`each`]: crate::each
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
    /// Create a new [`CombinatorMatcher`].
    ///
    /// The `mode` parameter determines whether this works like [`any`] or like [`each`].
    ///
    /// [`any`]: crate::any
    /// [`each`]: crate::each
    pub fn new(mode: CombinatorMode, block: impl FnOnce(&mut CombinatorContext<T>) + 'a) -> Self {
        Self {
            mode,
            func: Box::new(block),
        }
    }
}

impl<'a, T> TransformMatch for CombinatorMatcher<'a, T> {
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
                    Ok(MatchOutcome::Success(ctx.value))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
            (Ok(failures), CombinatorMode::All) => {
                if failures.iter().any(Option::is_some) {
                    Ok(MatchOutcome::Fail(failures))
                } else {
                    Ok(MatchOutcome::Success(ctx.value))
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
                    Ok(MatchOutcome::Fail(failures))
                } else {
                    Ok(MatchOutcome::Success(ctx.value))
                }
            }
            (Ok(failures), CombinatorMode::All) => {
                if failures.iter().any(Option::is_none) {
                    Ok(MatchOutcome::Success(ctx.value))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
            (Err(error), _) => Err(error),
        }
    }
}
