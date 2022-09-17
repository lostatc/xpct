use std::convert::Infallible;
use std::marker::PhantomData;

use crate::{
    core::{MatchBase, MatchNeg, MatchPos, MatchResult},
    fail, success,
};

#[derive(Debug)]
pub struct BeSomeMatcher<T> {
    marker: PhantomData<T>,
}

impl<T> BeSomeMatcher<T> {
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

impl<T> MatchBase for BeSomeMatcher<T> {
    type In = Option<T>;
}

impl<T> MatchPos for BeSomeMatcher<T> {
    type PosOut = T;
    type PosFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match actual {
            Some(value) => success!(value),
            None => fail!(()),
        }
    }
}

impl<T> MatchNeg for BeSomeMatcher<T> {
    type NegOut = Option<Infallible>;
    type NegFail = ();

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match actual {
            Some(_) => fail!(()),
            None => success!(None),
        }
    }
}

#[derive(Debug)]
pub struct BeNoneMatcher<T> {
    marker: PhantomData<T>,
}

impl<T> BeNoneMatcher<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T> Default for BeNoneMatcher<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MatchBase for BeNoneMatcher<T> {
    type In = Option<T>;
}

impl<T> MatchPos for BeNoneMatcher<T> {
    type PosOut = Option<Infallible>;
    type PosFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match actual {
            Some(_) => fail!(()),
            None => success!(None),
        }
    }
}

impl<T> MatchNeg for BeNoneMatcher<T> {
    type NegOut = T;
    type NegFail = ();

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match actual {
            Some(value) => success!(value),
            None => fail!(()),
        }
    }
}
