use std::marker::PhantomData;

use super::error::{DynMatchError, MatchError};
use super::format::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchCase {
    /// We are expecting this matcher to match.
    Positive,
    
    /// We are expecting this matcher to not match.
    Negative,
}

impl MatchCase {
    pub fn is_positive(&self) -> bool {
        match self {
            Self::Positive => true,
            Self::Negative => false,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Self::Positive => false,
            Self::Negative => true,
        }
    }
}

#[derive(Debug)]
pub struct MatchContext {
    case: MatchCase
}

impl MatchContext {
    pub(super) fn new(case: MatchCase) -> Self {
        Self { case }
    }

    pub fn case(&self) -> MatchCase {
        self.case
    }
}

pub trait Match {
    type In;
    type Out;
    type Reason;

    fn matches(&mut self, ctx: &MatchContext, actual: Self::In) -> Result<Self::Out, MatchError<Self::Reason>>;
}

pub trait DynMatch {
    type In;
    type Out;

    fn matches(&mut self, ctx: &MatchContext, actual: Self::In) -> Result<Self::Out, DynMatchError>;
}

struct InnerMatcher<M, ReasonFmt, ErrorFmt>
where
    M: Match,
    ReasonFmt: Display + From<M::Reason>,
    ErrorFmt: Display + From<anyhow::Error>,
{
    matcher: M,
    reason_fmt: PhantomData<ReasonFmt>,
    error_fmt: PhantomData<ErrorFmt>,
}

impl<M, ReasonFmt, ErrorFmt> InnerMatcher<M, ReasonFmt, ErrorFmt>
where
    M: Match,
    ReasonFmt: Display + From<M::Reason>,
    ErrorFmt: Display + From<anyhow::Error>,
{
    pub fn new(matcher: M) -> Self {
        Self {
            matcher,
            reason_fmt: PhantomData,
            error_fmt: PhantomData,
        }
    }
}

impl<M, ReasonFmt, ErrorFmt> DynMatch for InnerMatcher<M, ReasonFmt, ErrorFmt>
where
    M: Match,
    ReasonFmt: Display + From<M::Reason> + 'static,
    ErrorFmt: Display + From<anyhow::Error> + 'static,
{
    type In = M::In;
    type Out = M::Out;
                    
    fn matches(&mut self, ctx: &MatchContext, actual: Self::In) -> Result<Self::Out, DynMatchError> {
        match self.matcher.matches(ctx, actual) {
            Ok(out) => Ok(out),
            Err(error) => Err(DynMatchError::new::<M::Reason, ReasonFmt, ErrorFmt>(error)),
        }
    }
}

pub struct Matcher<In, Out>(Box<dyn DynMatch<In = In, Out = Out>>);

impl<In, Out> Matcher<In, Out> {
    pub fn new<M, ReasonFmt, ErrorFmt>(matcher: M) -> Self
    where
        M: Match<In = In, Out = Out> + 'static,
        ReasonFmt: Display + From<M::Reason> + 'static,
        ErrorFmt: Display + From<anyhow::Error> + 'static,
    {
        Self(Box::new(InnerMatcher::<M, ReasonFmt, ErrorFmt>::new(matcher)))
    }
}

impl<In, Out> DynMatch for Matcher<In, Out> {
    type In = In;
    type Out = Out;
    
    fn matches(&mut self, ctx: &MatchContext, actual: Self::In) -> Result<Self::Out, DynMatchError> {
        self.0.matches(ctx, actual)
    }
}
