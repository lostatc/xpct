//! The matchers provided by this crate.

mod boolean;
mod casefold;
mod chain;
mod combinator;
mod contain;
mod default;
mod diff;
mod elements;
mod equal;
mod every;
mod fields;
mod file;
mod float;
mod json;
mod len;
mod map;
mod not;
mod option;
mod ord;
mod pattern;
mod regex;
mod result;
mod substr;
mod time;
mod zero;

#[cfg(feature = "regex")]
pub use self::regex::RegexMatcher;
pub use boolean::BeTrueMatcher;
#[cfg(feature = "casefold")]
pub use casefold::EqCasefoldMatcher;
pub use chain::{ChainAssertion, ChainMatcher};
pub use combinator::{
    CombinatorAssertion, CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures,
};
pub use contain::{BeInMatcher, ConsistOfMatcher, ContainElementsMatcher, Contains};
pub use default::BeDefaultMatcher;
#[cfg(feature = "diff")]
pub use diff::{Diff, DiffEqualMatcher, DiffSegment, DiffTag, Diffable};
pub use elements::MatchElementsMatcher;
pub use equal::{EqualMatcher, Mismatch};
pub use every::EveryMatcher;
#[doc(hidden)]
pub use fields::__FieldsSpecParams;
pub use fields::{FailuresByField, FieldMatcher, FieldsSpec};
pub use file::{FileExistsMatcher, FileExistsMode};
#[cfg(feature = "float")]
pub use float::ApproxEqFloatMatcher;
#[cfg(feature = "json")]
pub use json::JsonMatcher;
pub use len::{BeEmptyMatcher, HaveLenMatcher, Len};
pub use map::{IterMap, IterMapMatcher, IterTryMapMatcher, MapMatcher, TryMapMatcher};
pub use not::NotMatcher;
pub use option::{BeSomeMatcher, Expectation};
pub use ord::{BeSortedByMatcher, BeSortedMatcher, Inequality, OrdMatcher, SortOrder};
pub use pattern::{Pattern, PatternMatcher};
pub use result::BeOkMatcher;
pub use substr::{ContainSubstrMatcher, HavePrefixMatcher, HaveSuffixMatcher};
pub use time::ApproxEqTimeMatcher;
pub use zero::{BeZeroMatcher, NonZeroInt};
