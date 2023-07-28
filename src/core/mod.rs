//! Core types for the library.
//!
//! This module contains everything that isn't a matcher or formatter. If you're just writing tests
//! and not writing custom matchers or formatters, you don't need anything in this module.

mod adapter;
mod assertion;
mod context;
mod format;
mod matcher;
mod result;
mod wrap;

pub use assertion::Assertion;
pub use context::{AssertionContext, FileLocation};
pub use format::*;
pub use matcher::{BoxTransformMatch, DynTransformMatch, Matcher, SimpleMatch, TransformMatch};
pub use result::{AssertionFailure, FormattedFailure, MatchError, MatchFailure, MatchOutcome};
