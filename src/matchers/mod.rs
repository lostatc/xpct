//! The matchers provided by this crate.

mod boolean;
mod chain;
mod combinator;
mod contain;
mod default;
mod equal;
mod every;
mod fields;
mod json;
mod len;
mod map;
mod not;
mod option;
mod ord;
mod regex;
mod result;
mod substr;

#[cfg(feature = "regex")]
pub use self::regex::MatchRegexMatcher;
pub use boolean::BeTrueMatcher;
pub use chain::{ChainAssertion, ChainMatcher};
pub use combinator::{
    CombinatorAssertion, CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures,
};
pub use contain::{BeInMatcher, ConsistOfMatcher, ContainElementsMatcher, Contains};
pub use default::BeDefaultMatcher;
pub use equal::{EqualMatcher, Mismatch};
pub use every::EveryMatcher;
pub use fields::{FailuresByField, FieldMatcher, FieldMatcherSpec};
#[cfg(feature = "json")]
pub use json::MatchJsonMatcher;
pub use len::{HaveLenMatcher, Len};
pub use map::{IterMapMatcher, IterTryMapMatcher, MapMatcher, TryMapMatcher};
pub use not::NotMatcher;
pub use option::BeSomeMatcher;
pub use ord::{BeSortedByMatcher, BeSortedMatcher, Inequality, OrdMatcher, SortOrder};
pub use result::BeOkMatcher;
pub use substr::{ContainSubstrMatcher, HavePrefixMatcher, HaveSuffixMatcher};
