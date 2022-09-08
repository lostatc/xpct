pub use matcher::*;

pub mod matchers;

mod assertion;
mod context;
mod format;
mod indent;
mod matcher;
mod result;

pub use assertion::{expect, Assertion};
pub use context::{AssertionContext, FileLocation};
pub use format::{AssertionFormat, DefaultAssertionFormat, Format, Formatter, ResultFormat};
pub use matcher::{
    DynMap, DynMapNeg, DynMapPos, DynMatch, DynMatchBase, MapNeg, MapPos, Match, MatchBase,
    MatchCase, Matcher, MapAdapter,
};
pub use result::{MatchError, MatchFailure, MatchResult};

pub mod prelude {
    pub use crate::expect;
    pub use crate::matchers::not;
}
