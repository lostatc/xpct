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
    DynMatch, MatchBase, MatchNeg, DynMatchNeg, MatchPos, DynMatchPos,
    Matcher, BoxMatcher,
};
pub use result::{MatchError, MatchFailure, DynMatchFailure, MatchResult};

pub mod prelude {
    pub use crate::expect;
    pub use crate::matchers::{not, any, all, each, equal};
}
