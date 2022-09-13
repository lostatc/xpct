#![cfg(feature = "fmt")]

use std::convert::Infallible;

use crate::{AssertionContext, AssertionFailure};

use super::{AssertionFormat, Format, Formatter};

#[derive(Debug)]
pub struct DefaultAssertionFormat;

impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
}
