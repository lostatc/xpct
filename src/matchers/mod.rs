//! The matchers provided by this crate.

/// Matchers for working with booleans.
pub mod boolean;
/// Matchers for working with collections.
pub mod collections;
/// Combinator matchers for composing other matchers.
pub mod combinators;
/// Matchers for working with `Default` values.
pub mod default;
/// Matchers that generate diffs between values.
#[cfg(feature = "diff")]
pub mod diff;
#[cfg(feature = "diff")]
mod diff_impl;
/// Matchers for comparing if two values are equal.
pub mod equal;
/// Matchers for making assertions about struct fields.
pub mod fields;
/// Matchers for working with files.
pub mod files;
/// Matchers that map values in a matcher pipeline.
pub mod map;
/// Matchers that invert other matchers.
pub mod not;
/// Matchers for working with numeric values.
pub mod numbers;
/// Matchers for working with `Option` values.
pub mod option;
/// Matchers for making assertions about the ordering of values.
pub mod ord;
/// Matchers for making assertions using patterns.
pub mod pattern;
/// Matchers for working with `Result` values.
pub mod result;
/// Matchers for working with strings.
pub mod strings;
/// Matchers for working with time.
pub mod time;
mod values;

pub use values::{Expectation, FailuresByField, Mismatch, SomeFailures};
