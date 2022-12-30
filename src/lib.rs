/*!
xpct is an assertions library for Rust.

It's designed to be ergonomic, batteries-included, and test framework agnostic.

If you're new here, you may want to check out the [Tutorial][crate::docs::tutorial] and the rest of
the [User Docs][crate::docs].
*/

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
// Disabling this feature does not change the public API, but the documentation makes it appear as
// if it does.
#![cfg_attr(docsrs, doc(cfg_hide(feature = "color")))]
// While technically accurate and potentially helpful, this feature flag adds a lot of unnecessary
// noise to the API docs.
#![cfg_attr(docsrs, doc(cfg_hide(feature = "fmt")))]

// Test code snippets in the README.

#[cfg(doctest)]
use doc_comment::doctest;

#[cfg(all(doctest, feature = "regex"))]
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
