mod assertion;
mod context;
mod format;
mod matcher;
mod result;

pub(super) mod style;

pub use format::*;

pub use matcher::{
    BoxMatch, BoxMatchNeg, BoxMatchPos, DynMatch, DynMatchNeg, DynMatchPos, MatchBase, MatchNeg,
    MatchPos, Matcher, NegMatcher, PosMatcher, SimpleMatch,
};

pub use result::{AssertionFailure, DynMatchFailure, MatchError, MatchFailure, MatchResult};

pub use assertion::{expect, Assertion};

pub use context::{AssertionContext, FileLocation};
