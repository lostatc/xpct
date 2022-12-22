//! The matchers provided by this crate.

mod boolean;
mod chain;
mod combinator;
mod default;
mod equal;
mod fields;
mod map;
mod not;
mod option;
mod ord;
mod result;
mod substr;

pub use boolean::BeTrueMatcher;
pub use chain::{ChainAssertion, ChainMatcher};
pub use combinator::{
    CombinatorAssertion, CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures,
};
pub use default::BeDefaultMatcher;
pub use equal::{EqualMatcher, Mismatch};
pub use fields::{FailuresByField, FieldMatcher, FieldMatcherSpec};
pub use map::{MapMatcher, TryMapMatcher};
pub use not::NotMatcher;
pub use option::BeSomeMatcher;
pub use ord::{Inequality, OrdMatcher};
pub use result::BeOkMatcher;
pub use substr::ContainSubstrMatcher;
