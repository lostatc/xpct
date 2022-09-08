use core::fmt;

use super::format::{Format, Formatter, ResultFormat};

pub struct MatchFailure(Box<dyn Format>);

impl MatchFailure {
    pub(crate) fn success<Success, Fmt>(success: Success) -> Self
    where
        Fmt: ResultFormat<Success = Success>,
    {
        Self(Box::new(Fmt::from(MatchResult::Success(success))))
    }

    pub(crate) fn fail<Fail, Fmt>(fail: Fail) -> Self
    where
        Fmt: ResultFormat<Fail = Fail>,
    {
        Self(Box::new(Fmt::from(MatchResult::Fail(fail))))
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
