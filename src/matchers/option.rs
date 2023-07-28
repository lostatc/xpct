use std::convert::Infallible;
use std::marker::PhantomData;

use crate::core::{Match, MatchOutcome};

/// A value that is returned by matchers when the actual value doesn't meet some criteria.
///
/// This is meant to be a deliberately generic value that can be reused in a number of different
/// matchers and formatted with [`ExpectationFormat`].
///
/// This can be used for matchers like [`be_some`] and [`be_ok`] to represent a value not meeting
/// some criteria, such as not being `Some(_)` and not being `Ok(_)` respectively.
///
/// Use this over [`Mismatch`] when there's only one case in which the matcher could fail.
///
/// When returned by a matcher, this value just means, "here is the actual value." It's up to the
/// formatter to determine how that information is presented to the user.
///
/// [`ExpectationFormat`]: crate::format::ExpectationFormat
/// [`Mismatch`]: crate::matchers::Mismatch
/// [`be_some`]: crate::be_some
/// [`be_ok`]: crate::be_ok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expectation<Actual> {
    /// The actual value.
    pub actual: Actual,
}

/// The matcher for [`be_some`] and [`be_none`].
///
/// [`be_some`]: crate::be_some
/// [`be_none`]: crate::be_none
#[derive(Debug, Default)]
pub struct BeSomeMatcher<T> {
    marker: PhantomData<T>,
}

impl<T> BeSomeMatcher<T> {
    /// Create a new [`BeSomeMatcher`].
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T> Match for BeSomeMatcher<T> {
    type In = Option<T>;

    type PosOut = T;
    type NegOut = Option<Infallible>;

    type PosFail = Expectation<Option<T>>;
    type NegFail = Expectation<Option<T>>;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match actual {
            Some(value) => Ok(MatchOutcome::Success(value)),
            None => Ok(MatchOutcome::Fail(Expectation { actual: None })),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match actual {
            Some(value) => Ok(MatchOutcome::Fail(Expectation {
                actual: Some(value),
            })),
            None => Ok(MatchOutcome::Success(None)),
        }
    }
}
