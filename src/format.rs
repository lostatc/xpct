use std::fmt;

#[cfg(feature = "handlebars")]
use serde::Serialize;

#[cfg(feature = "fmt")]
use super::context::AssertionContext;

use super::result::{MatchError, MatchFailure};

pub trait ResultFormat: fmt::Display {
    type Pos;
    type Neg;

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self;
}

#[derive(Debug)]
#[cfg_attr(feature = "handlebars", derive(Serialize))]
pub struct AssertionFailure<Context> {
    pub ctx: Context,
    pub error: MatchError,
}

pub trait AssertionFormat: fmt::Display {
    type Context;

    fn new(fail: AssertionFailure<Self::Context>) -> Self;
}

#[cfg(feature = "fmt")]
#[derive(Debug)]
pub struct DefaultAssertionFormat(AssertionFailure<AssertionContext>);

#[cfg(feature = "fmt")]
impl fmt::Display for DefaultAssertionFormat {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(feature = "fmt")]
impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;

    fn new(fail: AssertionFailure<Self::Context>) -> Self {
        Self(fail)
    }
}
