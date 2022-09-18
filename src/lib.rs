//! xpct is an assertions library for Rust.
//!
//! It's designed to be ergonomic, batteries-included, and test framework agnostic.
//!
//! # Tutorial
//!
//! To make an assertion, you'll usually start with the [`expect`] macro:
//!
//! ```
//! use xpct::{expect, equal};
//!
//! expect!("disco").to(equal("disco"));
//! ```
//!
//! In the above example, [`equal`] is a *matcher*. This crate provides a number of matchers in the
//! crate root, and you can implement custom matchers as well.
//!
//! When an assertion fails, it panics with an error message.
//!
//! You can also chain matchers like this:
//!
//! ```
//! use xpct::{expect, be_gt, be_lt};
//!
//! expect!(41)
//!     .to(be_gt(0)) // 41 > 0
//!     .to(be_lt(57)); // 41 < 57
//! ```
//!
//! When you chain together multiple matchers like this, the assertion only succeeds if *all* of
//! them match.
//!
//! You can also negate matchers by calling `to_not` or using the [`not`] matcher:
//!
//! ```
//! use xpct::{expect, equal, not};
//!
//! // These are equivalent.
//! expect!(41).to_not(equal(57));
//! expect!(41).to(not(equal(57)));
//! ```
//!
//! Not all matchers can be negated like this; matchers that can be negated will return a
//! [`Matcher`], while matchers that cannot be negated will return a [`PosMatcher`]. You'll see the
//! terms "pos" and "neg," short for *positive* and *negative*, throughout the API a lot. These
//! refer to whether a matcher is negated (negative) or not negated (positive).
//!
//! When you chain together matchers, they pass the value you passed to `expect!` into the next
//! matcher in the chain. Matchers can change the type of this value, which allows some matchers to
//! do things like unwrap [`Result`] and [`Option`] types.
//!
//! ```
//! use xpct::{expect, equal, be_ok};
//!
//! fn might_fail() -> anyhow::Result<String> {
//!     Ok(String::from("Whirling-in-Rags"))
//! }
//!
//! expect!(might_fail())
//!     .to(be_ok())
//!     .to(equal("Whirling-in-Rags"));
//! ```
//!
//! In the above example, we don't need to unwrap the [`Result`], because the [`be_ok`] matcher did
//! it for us! If we were to negate this matcher with [`not`], then it would return the value of
//! the [`Err`] variant instead.
//!
//! You can always get the value back out at the end by calling [`Assertion::into_inner`].
//!
//! ```
//! use xpct::{expect, be_some};
//!
//! let name: &'static str = expect!(Some("Raphaël Ambrosius Costeau"))
//!     .to(be_some())
//!     .into_inner();
//! ```
//!
//! There are combinator matchers like [`all`], [`each`], and [`any`] which allow us to combine
//! matchers in different ways:
//!
//! ```
//! use xpct::{expect, any, equal, be_none, matchers::EachContext};
//!
//! fn necktie_kind() -> Option<String> {
//!     None
//! }
//!
//! expect!(necktie_kind()).to(any(|ctx| {
//!     ctx.map(Option::as_deref)
//!         .to(be_none())
//!         .to(equal(Some("horrific")));
//! }));
//!
//! ```
//!
//! If you want to attach additional context to a matcher to include in the failure output, you can
//! use [`why`] and [`why_lazy`]:
//!
//! ```
//! use xpct::{expect, why, not, equal};
//!
//! expect!("Kim Kitsuragi").to(why(
//!     not(equal("kim kitsuragi")),
//!     "names should be capitalized"
//! ));
//! ```
//!
//! If you want to match on multiple fields of a struct, rather than using a separate `expect!`
//! assertion for each field, you can use [`match_fields`] with the [`fields`] macro.
//!
//! ```
//! use xpct::{expect, match_fields, fields, equal, be_none, be_ge, be_true};
//!
//! struct Person {
//!     name: Option<String>,
//!     id: String,
//!     age: u32,
//!     is_disco: bool,
//! }
//!
//! let value = Person {
//!     name: None,
//!     id: String::from("LTN-2JFR"),
//!     age: 44,
//!     is_disco: true,
//! };
//!
//! expect!(value).to(match_fields(fields!(Person {
//!     name: be_none(),
//!     id: equal("LTN-2JFR"),
//!     age: be_ge(44),
//!     is_disco: be_true(),
//! })));
//! ```
//!
//! [`Matcher`]: crate::core::Matcher
//! [`PosMatcher`]: crate::core::PosMatcher
//! [`Assertion::into_inner`]: crate::core::Assertion::into_inner
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod core;
pub mod matchers;

#[cfg(feature = "fmt")]
pub mod format;

#[cfg(feature = "fmt")]
pub use format::matchers::*;
