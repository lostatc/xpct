pub mod matchers;

mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod template;
mod color;

pub use assertion::{expect, Assertion};
pub use context::{AssertionContext, FileLocation};
pub use format::{Format, AssertionFormat, ResultFormat, OutputStream, Formatter};
pub use matcher::{
    BoxMatcher, DynMatch, DynMatchNeg, DynMatchPos, MatchBase, MatchNeg, MatchPos, Matcher,
    SimpleMatch,
};
pub use result::{AssertionFailure, DynMatchFailure, MatchError, MatchFailure, MatchResult};

#[cfg(feature = "fmt")]
pub use format::DefaultAssertionFormat;

#[cfg(feature = "color")]
pub use color::{OutputStyle, TerminalColor, TextColor, TextStyle};

pub mod prelude {
    #[cfg(feature = "fmt")]
    pub use {
        crate::expect,
        crate::matchers::{all, any, each, equal, not},
    };
}
