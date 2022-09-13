// TODO: Remove
#![allow(dead_code)]

pub mod matchers;

mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod template;

use static_assertions::{assert_impl_all, assert_obj_safe};

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

assert_impl_all!(Formatter: std::fmt::Debug, std::fmt::Write);
assert_impl_all!(FormattedOutput: std::fmt::Debug, std::fmt::Display);
assert_obj_safe!(
    DynMatchPos<In = (), PosOut = ()>,
    DynMatchNeg<In = (), NegOut = ()>,
    DynMatch<In = (), PosOut = (), NegOut = ()>,
);
