#![cfg(feature = "fmt")]

use std::marker::PhantomData;

use crate::{Format, Formatter, DynMatchFailure, MatchFailure, ResultFormat};

use super::{AllFailures, Mismatch, SomeFailures};

pub struct AllFailuresFormat;

impl Format for AllFailuresFormat {
    type Value = AllFailures;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

pub struct SomeFailuresFormat;

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

#[derive(Debug)]
pub struct AnyFormat;

impl Format for AnyFormat {
    type Value = MatchFailure<AllFailures, SomeFailures>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;
}

#[derive(Debug)]
pub struct AllFormat;

impl Format for AllFormat {
    type Value = MatchFailure<DynMatchFailure, ()>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl ResultFormat for AllFormat {
    type Pos = DynMatchFailure;
    type Neg = ();
}

#[derive(Debug)]
pub struct EachFormat;

impl Format for EachFormat {
    type Value = MatchFailure<DynMatchFailure, ()>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl ResultFormat for EachFormat {
    type Pos = DynMatchFailure;
    type Neg = ();
}

#[derive(Debug)]
pub struct EqualFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> Default for EqualFormat<Actual, Expected> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<Actual, Expected> EqualFormat<Actual, Expected> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<Actual, Expected> Format for EqualFormat<Actual, Expected> {
    type Value = MatchFailure<Mismatch<Actual, Expected>>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl<Actual, Expected> ResultFormat for EqualFormat<Actual, Expected> {
    type Pos = Mismatch<Actual, Expected>;
    type Neg = Mismatch<Actual, Expected>;
}

#[derive(Debug)]
pub struct NotFormat;

impl Format for NotFormat {
    type Value = MatchFailure<DynMatchFailure>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl ResultFormat for NotFormat {
    type Pos = DynMatchFailure;
    type Neg = DynMatchFailure;
}
