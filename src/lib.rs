#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod core;
pub mod matchers;

#[cfg(feature = "fmt")]
pub mod format;

#[cfg(feature = "fmt")]
pub use format::matchers::*;
