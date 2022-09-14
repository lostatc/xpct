mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod template;

pub(super) mod style;

pub use format::*;

pub use matcher::{
    BoxMatcher, DynMatch, DynMatchNeg, DynMatchPos, MatchBase, MatchNeg, MatchPos, Matcher,
    SimpleMatch,
};

pub use result::{AssertionFailure, DynMatchFailure, MatchError, MatchFailure, MatchResult};

pub use assertion::{expect, Assertion};

pub use context::{AssertionContext, FileLocation};
