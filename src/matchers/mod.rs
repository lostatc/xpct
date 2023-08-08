//! The matchers provided by this crate.
//!
//! This module provides the underlying matcher implementations for the matcher functions exported
//! at the crate root. For example, this module provides [`EqualMatcher`], which is used to
//! implement [`equal`].
//!
//! If you're just writing tests and not writing custom matchers or formatters, you don't need
//! anything in this module.
//!
//! If you want to change the formatting of one of the provided matchers, you can re-use the matcher
//! implementation in this module and pair it with your own custom formatter.
//!
//! See [Writing Custom Formatters][crate::docs::writing_formatters] to learn how to change the
//! formatting of the provided matchers.
//!
//! See [Writing Custom Matchers][crate::docs::writing_matchers] to learn how to implement your own
//! matchers like the ones in this module.
//!
//! [`EqualMatcher`]: crate::matchers::equal::EqualMatcher
//! [`equal`]: crate::equal

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
