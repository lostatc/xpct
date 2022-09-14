use std::fmt;

use super::{FormattedOutput, ResultFormat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchFailure<Pos, Neg = Pos> {
    // We were expecting the matcher to succeed but it failed.
    Pos(Pos),

    // We were expecting the matcher to fail but it succeeded.
    Neg(Neg),
}

impl<Pos, Neg> Into<FormattedOutput> for MatchFailure<Pos, Neg>
where
    Pos: Into<FormattedOutput>,
    Neg: Into<FormattedOutput>,
{
    fn into(self) -> FormattedOutput {
        match self {
            MatchFailure::Pos(fail) => fail.into(),
            MatchFailure::Neg(fail) => fail.into(),
        }
    }
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

#[derive(Debug)]
pub struct DynMatchFailure(FormattedOutput);

impl DynMatchFailure {
    pub fn new<Fmt, PosFail, NegFail>(
        fail: MatchFailure<PosFail, NegFail>,
        format: Fmt,
    ) -> Result<Self, Fmt::Error>
    where
        Fmt: ResultFormat<Pos = PosFail, Neg = NegFail>,
    {
        Ok(Self(FormattedOutput::new(fail, format)?))
    }

    pub fn into_fmt(self) -> FormattedOutput {
        self.0
    }
}

impl From<DynMatchFailure> for FormattedOutput {
    fn from(fail: DynMatchFailure) -> Self {
        fail.0
    }
}

impl fmt::Display for DynMatchFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DynMatchFailure {}

#[derive(Debug)]
pub struct AssertionFailure<Context> {
    pub ctx: Context,
    pub error: MatchError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult<Success, Fail> {
    Success(Success),
    Fail(Fail),
}

impl<Success, Fail> MatchResult<Success, Fail> {
    pub fn is_success(&self) -> bool {
        match self {
            MatchResult::Success(_) => true,
            MatchResult::Fail(_) => false,
        }
    }

    pub fn is_fail(&self) -> bool {
        match self {
            MatchResult::Success(_) => false,
            MatchResult::Fail(_) => true,
        }
    }
}

impl<Success, Fail> From<MatchResult<Success, Fail>> for Result<Success, Fail> {
    fn from(result: MatchResult<Success, Fail>) -> Self {
        match result {
            MatchResult::Success(success) => Ok(success),
            MatchResult::Fail(fail) => Err(fail),
        }
    }
}

#[derive(Debug)]
pub enum MatchError {
    Fail(DynMatchFailure),
    Err(anyhow::Error),
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchError::Fail(fail) => fail.fmt(f),
            MatchError::Err(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for MatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MatchError::Fail(_) => None,
            MatchError::Err(error) => error.source(),
        }
    }
}

#[macro_export]
macro_rules! success {
    ($success:expr) => {
        return std::result::Result::Ok($crate::MatchResult::Success($success.into()))
    };
}

#[macro_export]
macro_rules! fail {
    ($fail:expr) => {
        return std::result::Result::Ok($crate::MatchResult::Fail($fail.into()))
    };
}
