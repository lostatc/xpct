#[cfg(feature = "fmt")]
use super::context::AssertionContext;
use super::result::{AssertionFailure, MatchFailure};

pub trait Format {
    type Value;

    fn fmt(&self, value: Self::Value) -> String;
}

pub trait ResultFormat: Format<Value = MatchFailure<Self::Pos, Self::Neg>> {
    type Pos;
    type Neg;
}

pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    type Context;
}

#[cfg(feature = "fmt")]
#[derive(Debug)]
pub struct DefaultAssertionFormat;

#[cfg(feature = "fmt")]
impl Format for DefaultAssertionFormat {
    type Value = AssertionFailure<AssertionContext>;

    fn fmt(&self, _: Self::Value) -> String {
        todo!()
    }
}

#[cfg(feature = "fmt")]
impl AssertionFormat for DefaultAssertionFormat {
    type Context = AssertionContext;
}
