use std::fmt;

use super::{FormattedOutput, ResultFormat};

/// The result of a failed matcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchFailure<Pos, Neg = Pos> {
    /// We were expecting the matcher to succeed but it failed.
    Pos(Pos),

    /// We were expecting the matcher to fail but it succeeded.
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
    /// This is a [`MatchFailure::Pos`].
    pub fn is_pos(&self) -> bool {
        match self {
            Self::Pos(_) => true,
            Self::Neg(_) => false,
        }
    }

    /// This is a [`MatchFailure::Neg`].
    pub fn is_neg(&self) -> bool {
        match self {
            Self::Pos(_) => false,
            Self::Neg(_) => true,
        }
    }
}

/// A [`MatchFailure`] which has been formatted with a [`ResultFormat`].
#[derive(Debug)]
pub struct FormattedFailure(FormattedOutput);

impl FormattedFailure {
    /// Create a new [`MatchFailure`] by formatting `fail` with `format`.
    pub fn new<Fmt, PosFail, NegFail>(
        fail: MatchFailure<PosFail, NegFail>,
        format: Fmt,
    ) -> anyhow::Result<Self>
    where
        Fmt: ResultFormat<Pos = PosFail, Neg = NegFail>,
    {
        Ok(Self(FormattedOutput::new(fail, format)?))
    }

    /// Convert this value into a [`FormattedOutput`].
    pub fn into_fmt(self) -> FormattedOutput {
        self.0
    }
}

impl From<FormattedFailure> for FormattedOutput {
    fn from(fail: FormattedFailure) -> Self {
        fail.0
    }
}

impl fmt::Display for FormattedFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for FormattedFailure {}

/// The result of a failed assertion.
#[derive(Debug)]
pub struct AssertionFailure<Context> {
    /// A generic context value to associate with the assertion.
    ///
    /// This value can be use to capture context about the assertion. The provided
    /// [`DefaultAssertionFormat`] has a `Context` value of [`AssertionContext`], which captures
    /// the current file name, line and column number, and the stringified expression passed to
    /// [`expect!`].
    ///
    /// [`DefaultAssertionFormat`]: crate::core::DefaultAssertionFormat
    /// [`AssertionContext`]: crate::core::AssertionContext
    /// [`exepct!`]: crate::expect
    pub ctx: Context,

    /// The error that caused this assertion to fail.
    pub error: MatchError,
}

/// The result of a matcher, either `Succcess` or `Fail`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult<Success, Fail> {
    /// The matcher matched.
    Success(Success),

    /// The matcher did not match.
    Fail(Fail),
}

impl<Success, Fail> MatchResult<Success, Fail> {
    /// This is a [`MatchResult::Success`].
    pub fn is_success(&self) -> bool {
        match self {
            MatchResult::Success(_) => true,
            MatchResult::Fail(_) => false,
        }
    }

    /// This is a [`MatchResult::Fail`].
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

/// An error from a matcher, meaning it either failed or returned an error.
#[derive(Debug)]
pub enum MatchError {
    /// The matcher failed.
    ///
    /// This can either mean it was expecting to match but didn't, or it was expecting to not match
    /// but did.
    Fail(FormattedFailure),

    /// The matcher returned an error.
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

/// Return early from a matcher with [`MatchResult::Success`].
///
/// This can be used when implementing matchers with [`MatchPos`] and [`MatchNeg`].
///
/// [`MatchPos`]: crate::core::MatchPos
/// [`MatchNeg`]: crate::core::MatchNeg
#[macro_export]
macro_rules! success {
    ($success:expr) => {
        return ::std::result::Result::Ok($crate::core::MatchResult::Success(
            ::std::convert::Into::into($success),
        ))
    };
}

/// Return early from a matcher with [`MatchResult::Fail`].
///
/// This can be used when implementing matchers with [`MatchPos`] and [`MatchNeg`].
///
/// [`MatchPos`]: crate::core::MatchPos
/// [`MatchNeg`]: crate::core::MatchNeg
#[macro_export]
macro_rules! fail {
    ($fail:expr) => {
        return ::std::result::Result::Ok($crate::core::MatchResult::Fail(
            ::std::convert::Into::into($fail),
        ))
    };
}
