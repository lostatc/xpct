use crate::core::{AssertionFailure, MatchFailure};

use super::Formatter;

/// A generic formatter.
///
/// Implement this trait to create custom formatters. This works similarly to
/// [`std::fmt::Display`], where you call methods on the [`Formatter`] to write to the output.
pub trait Format {
    /// The value to format.
    type Value;

    /// Format the value.
    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()>;
}

/// A formatter for failed matchers.
///
/// This formatter is used to format the results of failed matchers.
///
/// This trait has a blanket implementation for types that implement [`Format`] where
/// [`Format::Value`] is a [`MatchFailure`], so you should never need to implement this trait
/// yourself.
pub trait ResultFormat: Format<Value = MatchFailure<Self::Pos, Self::Neg>> {
    /// The match result in the *positive* case (when we were expecting the matcher to succeed).
    type Pos;

    /// The match result in the *negative* case (when we were expecting the matcher to fail).
    type Neg;
}

impl<T, Pos, Neg> ResultFormat for T
where
    T: Format<Value = MatchFailure<Pos, Neg>>,
{
    type Pos = Pos;
    type Neg = Neg;
}

/// A formatter for failed assertions.
///
/// This formatter is used to determine the format of the assertion as a whole, as opposed to just
/// a specific matcher. When you use the [`expect!`] macro, you're implicitly using the provided
/// [`DefaultAssertionFormat`]. However, you can implement this trait and use the [`expect`]
/// function instead to customize how assertions are formatted.
///
/// This trait has a blanket implementation for types that implement [`Format`] where
/// [`Format::Value`] is an [`AssertionFailure`], so you should never need to implement this trait
/// yourself.
///
/// [`expect!`]: crate::expect
/// [`expect`]: crate::core::expect
/// [`DefaultAssertionFormat`]: crate::core::DefaultAssertionFormat
pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    /// The context value associated with the assertion.
    type Context;
}

impl<T, Context> AssertionFormat for T
where
    T: Format<Value = AssertionFailure<Context>>,
{
    type Context = Context;
}

/// A wrapper over a formatter that negates it.
///
/// This type is a [`ResultFormat`] that swaps the [`MatchFailure::Pos`] and [`MatchFailure::Neg`]
/// values, so if you want to write a negated version of a matcher (e.g. [`be_ok`] vs [`be_err`]),
/// you don't have to write two formatters.
///
/// [`be_ok`]: crate::be_ok
/// [`be_err`]: crate::be_err
#[derive(Debug, Default)]
pub struct NegFormat<Fmt>(pub Fmt);

impl<Fmt, Pos, Neg> Format for NegFormat<Fmt>
where
    Fmt: Format<Value = MatchFailure<Pos, Neg>>,
{
    type Value = MatchFailure<Neg, Pos>;

    fn fmt(self, f: &mut super::Formatter, value: Self::Value) -> crate::Result<()> {
        match value {
            MatchFailure::Pos(fail) => self.0.fmt(f, MatchFailure::Neg(fail)),
            MatchFailure::Neg(fail) => self.0.fmt(f, MatchFailure::Pos(fail)),
        }
    }
}

/// A formatter which dispatches to different formatters depending on whether the matcher is
/// negated or not.
///
/// This type is a [`ResultFormat`] that wraps two existing formatters, one for the positive case
/// (we expected the matcher to match) and one for the negative case (we expected the matcher to
/// fail). It dispatches to one of those formatters depending on whether the matcher is negated or
/// not.
#[derive(Debug)]
pub struct DispatchFormat<PosFmt, NegFmt> {
    pos_fmt: PosFmt,
    neg_fmt: NegFmt,
}

impl<PosFmt, NegFmt> DispatchFormat<PosFmt, NegFmt> {
    /// Create a new [`DispatchFormat`] from two existing formatters.
    ///
    /// This accepts a formatter for the positive case and the negative case respectively.
    pub fn new(pos_fmt: PosFmt, neg_fmt: NegFmt) -> Self {
        Self { pos_fmt, neg_fmt }
    }
}

impl<PosFmt, NegFmt, PosFail, NegFail> Format for DispatchFormat<PosFmt, NegFmt>
where
    PosFmt: Format<Value = MatchFailure<PosFail>>,
    NegFmt: Format<Value = MatchFailure<NegFail>>,
{
    type Value = MatchFailure<PosFail, NegFail>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        match value {
            MatchFailure::Pos(fail) => self.pos_fmt.fmt(f, MatchFailure::Pos(fail)),
            MatchFailure::Neg(fail) => self.neg_fmt.fmt(f, MatchFailure::Neg(fail)),
        }
    }
}
