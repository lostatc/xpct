#![cfg(feature = "fmt")]

use crate::{AssertionContext, AssertionFailure};

use super::{AssertionFormat, Format, Formatter};

#[derive(Debug)]
pub struct DefaultAssertionFormat;

impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) {
        todo!()
    }
}

impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
}
