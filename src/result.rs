use core::fmt;

use super::format::{Format, Formatter, ResultFormat};

pub struct MatchFailure(Box<dyn Format>);

impl MatchFailure {
    pub(crate) fn new_pos<Fail, Fmt>(fail: Fail) -> Self
    where
        Fmt: ResultFormat<PosFail = Fail>,
    {
        Self(Box::new(Fmt::from(MatchResult::Fail(fail))))
    }

    pub(crate) fn new_neg<Fail, Fmt>(success: Fail) -> Self
    where
        Fmt: ResultFormat<NegFail = Fail>,
    {
        Self(Box::new(Fmt::from(MatchResult::Success(success))))
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
pub enum MatchResult<T, Fail> {
    Success(T),
    Fail(Fail),
}

#[derive(Debug)]
pub enum MatchError {
    Fail(MatchFailure),
    Err(anyhow::Error),
}
