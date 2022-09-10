pub mod matchers;

mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod template;

pub use assertion::{expect, Assertion};
pub use context::{AssertionContext, FileLocation};
pub use format::{AssertionFailure, AssertionFormat, ResultFormat};
pub use matcher::{
    BoxMatcher, DynMatch, DynMatchNeg, DynMatchPos, MatchBase, MatchNeg, MatchPos, Matcher,
    SimpleMatch,
};
pub use result::{DynMatchFailure, MatchError, MatchFailure, MatchResult};

#[cfg(feature = "handlebars")]
pub use template::HandlebarsFormat;

#[cfg(feature = "fmt")]
pub use format::DefaultAssertionFormat;

pub mod prelude {
    pub use crate::expect;
    pub use crate::matchers::{all, any, each, equal, not};
}
