use std::convert::Infallible;
use std::marker::PhantomData;

use crate::{
    core::{Match, MatchOutcome},
    fail, success,
};

/// The matcher for [`be_some`] and [`be_none`].
///
/// [`be_some`]: crate::be_some
/// [`be_none`]: crate::be_none
#[derive(Debug)]
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

impl<T> Default for BeSomeMatcher<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Match for BeSomeMatcher<T> {
    type In = Option<T>;

    type PosOut = T;
    type NegOut = Option<Infallible>;

    type PosFail = ();
    type NegFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match actual {
            Some(value) => success!(value),
            None => fail!(()),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match actual {
            Some(_) => fail!(()),
            None => success!(None),
        }
    }
}
