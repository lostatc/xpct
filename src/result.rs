use core::fmt;

use super::format::{Format, Formatter, ResultFormat};
use super::matcher::{MatchPos, MatchNeg};

#[derive(Debug)]
pub enum MatchFailure<Pos, Neg = Pos> {
    Pos(Pos),
    Neg(Neg),
}

impl<Pos, Neg> MatchFailure<Pos, Neg> {
    pub fn is_pos(&self) -> bool {
        match self {
            Self::Pos(_) => true,
            Self::Neg(_) => false,
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::Pos(_) => false,
            Self::Neg(_) => true,
        }
    }
}

pub struct DynMatchFailure(Box<dyn Format>);

impl DynMatchFailure {
    pub fn new<Fmt, PosFail, NegFail>(fail: MatchFailure<PosFail, NegFail>) -> Self
    where
        Fmt: ResultFormat<Pos = PosFail, Neg = NegFail>,
    {
        Self(Box::new(Fmt::new(fail)))
    }
}

impl fmt::Debug for DynMatchFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchFailure").finish()
    }
}

impl Format for DynMatchFailure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub enum MatchResult<T, Fail> {
    Success(T),
    Fail(Fail),
}

#[derive(Debug)]
pub enum MatchError {
    Fail(DynMatchFailure),
    Err(anyhow::Error),
}
