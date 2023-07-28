use std::marker::PhantomData;

use crate::core::{MatchOutcome, TransformMatch};

use super::Expectation;

/// The matcher for [`be_ok`] and [`be_err`].
///
/// [`be_ok`]: crate::be_ok
/// [`be_err`]: crate::be_err
#[derive(Debug)]
pub struct BeOkMatcher<T, E> {
    marker: PhantomData<(T, E)>,
}

impl<T, E> BeOkMatcher<T, E> {
    /// Create a new [`BeOkMatcher`].
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T, E> Default for BeOkMatcher<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, E> TransformMatch for BeOkMatcher<T, E> {
    type In = Result<T, E>;

    type PosOut = T;
    type NegOut = E;

    type PosFail = Expectation<Result<T, E>>;
    type NegFail = Expectation<Result<T, E>>;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match actual {
            Ok(value) => Ok(MatchOutcome::Success(value)),
            Err(err) => Ok(MatchOutcome::Fail(Expectation { actual: Err(err) })),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match actual {
            Ok(value) => Ok(MatchOutcome::Fail(Expectation { actual: Ok(value) })),
            Err(error) => Ok(MatchOutcome::Success(error)),
        }
    }
}
