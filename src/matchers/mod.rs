//! The matchers provided by this crate.
//!
//! # Writing custom matchers
//!
//! If none of the provided matchers suit your needs, xpct allows you to write custom matchers.
//! There are a few ways to do this.
//!
//! The simplest way is to implement the [`SimpleMatch`] trait. Here's an implementation of the
//! [`equal`] matcher.
//!
//! ```
//! use xpct::core::SimpleMatch;
//! use xpct::matchers::Mismatch;
//!
//! pub struct EqualMatcher<Expected> {
//!     expected: Expected,
//! }
//!
//! impl<Expected> EqualMatcher<Expected> {
//!     pub fn new(expected: Expected) -> Self {
//!         Self { expected }
//!     }
//! }
//!
//! impl<Expected, Actual> SimpleMatch<Actual> for EqualMatcher<Expected>
//! where
//!     Actual: PartialEq<Expected> + Eq,
//! {
//!     // This is the type the matcher returns if it fails. This is used by the formatter to
//!     // generate pretty test failure output.
//!     type Fail = Mismatch<Expected, Actual>;
//!
//!     // This returns `true` if the matcher matches and `false` otherwise.
//!     fn matches(&mut self, actual: &Actual) -> anyhow::Result<bool> {
//!         Ok(actual == &self.expected)
//!     }
//!
//!     // This is called if the matcher fails, and returns a value which is sent to the formatter
//!     // to format the output.
//!     fn fail(self, actual: Actual) -> Self::Fail {
//!         Mismatch {
//!             actual,
//!             expected: self.expected,
//!         }
//!     }
//! }
//!
//! ```
//!
//! To make `EqualMatcher` into a matcher, you just need to wrap it with [`Matcher::simple`]. This
//! method also accepts the formatter which is used to format the output. Thankfully, you don't
//! need to write the formatting logic yourself to get pretty output! Because our matcher returns a
//! [`Mismatch`] when it fails, we can use any formatter which accepts a [`Mismatch`], like the
//! aptly named [`MismatchFormat`].
//!
//! ```
//! # use xpct::matchers::EqualMatcher;
//! use std::fmt;
//!
//! use xpct::expect;
//! use xpct::core::Matcher;
//! use xpct::format::MismatchFormat;
//!
//! pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
//! where
//!     Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
//!     Expected: fmt::Debug + 'a,
//! {
//!     Matcher::simple(
//!         EqualMatcher::new(expected),
//!         MismatchFormat::new("to equal", "to not equal"),
//!     )
//! }
//!
//! ```
//!
//! What if we wanted to make a matcher which is the negated version of `EqualMatcher`, like
//! `not_equal`? For a matcher created by implementing [`SimpleMatch`], we can call
//! [`Matcher::simple_neg`] to negate it.
//!
//! ```
//! # use xpct::matchers::EqualMatcher;
//! use std::fmt;
//!
//! use xpct::expect;
//! use xpct::core::Matcher;
//! use xpct::format::MismatchFormat;
//!
//! pub fn not_equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
//! where
//!     Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
//!     Expected: fmt::Debug + 'a,
//! {
//!     Matcher::simple_neg(
//!         EqualMatcher::new(expected),
//!         // Remember that we need to flip these cases, because `actual != expected` is now the
//!         // *positive* case and `actual == expected` is now the *negative* case.
//!         MismatchFormat::new("to not equal", "to equal"),
//!     )
//! }
//!
//! expect!("disco").to(not_equal("not disco"));
//! ```
//!
//! A major limitation of [`SimpleMatch`] is that it always returns the same value that was passed
//! in, hence the name "simple." If you have more complex needs for your matcher, like you need it
//! to transform the value like the [`be_some`] and [`be_ok`] matchers do, you can implement
//! [`MatchPos`] and optionally [`MatchNeg`].
//!
//! ```
//! use std::marker::PhantomData;
//!
//! use xpct::{success, fail};
//! use xpct::format::MessageFormat;
//! use xpct::core::{Matcher, MatchPos, MatchNeg, MatchBase, MatchResult, NegFormat};
//!
//! pub struct BeOkMatcher<T, E> {
//!     // Matchers created by implementing `MatchPos` and `MatchNeg` will often need to use
//!     // `PhantomData` so they know their input and output types.
//!     marker: PhantomData<(T, E)>,
//! }
//!
//! impl<T, E> BeOkMatcher<T, E> {
//!     pub fn new() -> Self {
//!         Self {
//!             marker: PhantomData,
//!         }
//!     }
//! }
//!
//!  // You always need to implement this trait; it's the parent trait of `MatchPos` and `MatchNeg`.
//! impl<T, E> MatchBase for BeOkMatcher<T, E> {
//!     // The type the matcher accepts.
//!     type In = Result<T, E>;
//! }
//!
//! impl<T, E> MatchPos for BeOkMatcher<T, E> {
//!     // The type this matcher returns and passes to the next matcher if it matches.
//!     type PosOut = T;
//!
//!     // The type that is passed to the formatter if the matcher fails. Because we don't have any
//!     // interesting information to provide other than "the value was not `Ok`", we just make
//!     // this a unit struct.
//!     type PosFail = ();
//!
//!     // This returns `MatchResult::Success` if the matcher matches and `MatchResult::Fail`
//!     // otherwise.
//!     fn match_pos(
//!         self,
//!         actual: Self::In,
//!     ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
//!         match actual {
//!             // These macros are just shortcuts for returning a `MatchResult`; you don't have to
//!             // use them.
//!             Ok(value) => success!(value),
//!             Err(_) => fail!(()),
//!         }
//!     }
//! }
//!
//! // Implementing this trait is optional. If you implement this trait, then the matcher can be
//! // negated. The output type for the negated case can be different; in this case, it returns the
//! // error type of the `Result`.
//! impl<T, E> MatchNeg for BeOkMatcher<T, E> {
//!     type NegOut = E;
//!     type NegFail = ();
//!
//!     fn match_neg(
//!         self,
//!         actual: Self::In,
//!     ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
//!         match actual {
//!             Ok(_) => fail!(()),
//!             Err(error) => success!(error),
//!         }
//!     }
//! }
//!
//! // `MessageFormat` is a simple formatter that just returns a static message in each case.
//! fn result_format() -> MessageFormat {
//!     MessageFormat::new("Expected this to be Ok.", "Expected this to be Err.")
//! }
//!
//! pub fn be_ok<'a, T, E>() -> Matcher<'a, Result<T, E>, T, E>
//! where
//!     T: 'a,
//!     E: 'a,
//! {
//!     // For matchers implemented with `MatchPos` and `MatchNeg`, you use `Matcher::new`.
//!     Matcher::new(BeOkMatcher::new(), result_format())
//! }
//!
//! pub fn be_err<'a, T, E>() -> Matcher<'a, Result<T, E>, E, T>
//! where
//!     T: 'a,
//!     E: 'a,
//! {
//!     // You can use `Matcher::neg` to negate a matcher created by implementing `MatchPos` and
//!     // `MatchNeg`. You can use `NegFormat` to negate a formatter.
//!     Matcher::neg(BeOkMatcher::new(), NegFormat(result_format()))
//! }
//! ```
//!
//! [`be_some`]: crate::be_some
//! [`be_ok`]: crate::be_ok
//! [`SimpleMatch`]: crate::core::SimpleMatch
//! [`equal`]: crate::equal
//! [`Matcher::new`]: crate::core::Matcher::new
//! [`MismatchFormat`]: crate::format::MismatchFormat
//! [`Matcher::simple`]: crate::core::Matcher::simple
//! [`Matcher::simple_neg`]: crate::core::Matcher::simple_neg
//! [`MatchPos`]: crate::core::MatchPos
//! [`MatchNeg`]: crate::core::MatchNeg

mod boolean;
mod chain;
mod combinator;
mod equal;
mod fields;
mod map;
mod not;
mod option;
mod ord;
mod result;

pub use boolean::BeTrueMatcher;
pub use chain::{ChainAssertion, ChainMatcher};
pub use combinator::{
    CombinatorAssertion, CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures,
};
pub use equal::{EqualMatcher, Mismatch};
pub use fields::{FailuresByField, FieldMatcher};
pub use map::{MapMatcher, TryMapMatcher};
pub use not::NotMatcher;
pub use option::BeSomeMatcher;
pub use ord::{Inequality, OrdMatcher};
pub use result::BeOkMatcher;
