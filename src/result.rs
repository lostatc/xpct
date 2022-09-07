use core::fmt;

use super::format::{Format, Formatter, ResultFormat};
use super::matcher::MatchCase;

pub trait Matches {
    fn matches(&self) -> bool;
}

pub struct MatchFailure(Box<dyn Format>);

impl MatchFailure {
    pub(crate) fn new<Res, Fmt>(reason: Res, case: MatchCase) -> Self
    where
        Res: Matches,
        Fmt: ResultFormat<Res = Res>,
    {
        Self(Box::new(Fmt::new(reason, case)))
    }
}

impl fmt::Debug for MatchFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchFailure").finish()
    }
}

impl Format for MatchFailure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub enum MatchResult<T, Res> {
    Success(T),
    Fail(Res),
}

#[derive(Debug)]
pub enum MatchError {
    Fail(MatchFailure),
    Err(anyhow::Error),
}
