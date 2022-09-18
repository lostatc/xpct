use std::marker::PhantomData;

use crate::{
    core::{MatchBase, MatchNeg, MatchPos, MatchResult},
    fail, success,
};

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

impl<T, E> MatchBase for BeOkMatcher<T, E> {
    type In = Result<T, E>;
}

impl<T, E> MatchPos for BeOkMatcher<T, E> {
    type PosOut = T;
    type PosFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match actual {
            Ok(value) => success!(value),
            Err(_) => fail!(()),
        }
    }
}

impl<T, E> MatchNeg for BeOkMatcher<T, E> {
    type NegOut = E;
    type NegFail = ();

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match actual {
            Ok(_) => fail!(()),
            Err(error) => success!(error),
        }
    }
}