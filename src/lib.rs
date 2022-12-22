/*!
xpct is an assertions library for Rust.

It's designed to be ergonomic, batteries-included, and test framework agnostic.

If you're new here, you may want to check out the [Tutorial][crate::docs::tutorial] and the rest of
the [User Docs][crate::docs].
*/

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Test code snippets in the README.

#[cfg(doctest)]
use doc_comment::doctest;

#[cfg(doctest)]
doctest!("../README.md");

pub mod core;
pub mod docs;
mod error;
pub mod matchers;

#[cfg(feature = "fmt")]
pub mod format;

#[cfg(feature = "fmt")]
pub use format::matchers::*;

pub use error::{Error, Result};
