use std::marker::PhantomData;

use crate::core::{Match, MatchOutcome};
use crate::{fail, success};

#[derive(Debug)]
pub struct BeOkMatcher<T, E> {
    marker: PhantomData<(T, E)>,
}

impl<T, E> BeOkMatcher<T, E> {
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

impl<T, E> Match for BeOkMatcher<T, E> {
    type In = Result<T, E>;

    type PosOut = T;
    type NegOut = E;

    type PosFail = ();
    type NegFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match actual {
            Ok(value) => success!(value),
            Err(_) => fail!(()),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match actual {
            Ok(_) => fail!(()),
            Err(error) => success!(error),
        }
    }
}
