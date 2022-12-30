//! Core types for the library.
//!
//! This module contains everything that isn't a matcher or formatter.

mod adapter;
mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod wrap;

pub use format::*;

pub use matcher::{BoxMatch, DynMatch, Match, Matcher, SimpleMatch};

pub use result::{AssertionFailure, FormattedFailure, MatchError, MatchFailure, MatchOutcome};

pub use assertion::{expect, Assertion};

pub use context::{AssertionContext, FileLocation};
