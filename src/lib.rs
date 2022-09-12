pub mod matchers;

mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod template;

pub use assertion::{expect, Assertion};
pub use context::{AssertionContext, FileLocation};
pub use matcher::{
    BoxMatcher, DynMatch, DynMatchNeg, DynMatchPos, MatchBase, MatchNeg, MatchPos, Matcher,
    SimpleMatch,
};
pub use result::{AssertionFailure, DynMatchFailure, MatchError, MatchFailure, MatchResult};

pub use format::{AssertionFormat, Format, FormattedOutput, Formatter, OutputStream, ResultFormat};

#[cfg(feature = "color")]
pub use format::{OutputStyle, TerminalColor, TextColor, TextStyle};

#[cfg(feature = "fmt")]
pub use format::DefaultAssertionFormat;

#[cfg(feature = "fmt")]
pub mod prelude {
    pub use crate::expect;
    pub use crate::matchers::{all, any, each, equal, not};
}
