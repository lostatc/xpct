use std::fmt;

use super::adapter::{DynMatchAdapter, NegMatchAdapter, SimpleMatchAdapter};
use super::wrap::MatchWrapper;
use super::{FormattedFailure, MatchOutcome, ResultFormat};

/// The trait which is implemented to create matchers.
///
/// This trait exposes all the features of matchers, but for simple matchers, it may be easier to
/// implement [`SimpleMatch`] instead.
pub trait Match {
    /// The type of the "actual" value passed into the matcher by [`expect!`].
    ///
    /// [`expect!`]: crate::expect
    type In;

    /// The output type of the matcher that is passed to subsequent matchers in the chain.
    ///
    /// This only needs to be different from [`Self::In`] if this is a matcher that transforms its
    /// value, like [`be_ok`] or [`be_some`].
    ///
    /// [`be_ok`]: crate::be_ok
    /// [`be_some`]: crate::be_some
    type PosOut;

    /// The output type of the matcher in the negative case.
    ///
    /// This is the same as [`Self::PosOut`], except this is for when a matcher is negated (we're
    /// expecting it to fail).
    type NegOut;

    /// The failure output that is passed to the formatter.
    ///
    /// This value describes the reason why the matcher failed in a presentation-agnostic way so
    /// that a formatter can use it to generate its own output format. Values like [`Mismatch`] are
    /// meant to be used for this.
    ///
    /// [`Mismatch`]: crate::matchers::Mismatch
    type PosFail;

    /// The failure output of the matcher in the negative case.
    ///
    /// This is the same as [`Self::PosFail`], except this is for when a matcher is negated (we're
    /// expecting it to fail).
    type NegFail;

    /// The function called to test whether a value matches.
    ///
    /// This returns a [`MatchOutcome`], which determines whether the matcher succeeded or failed.
    /// It can also return an [`Err`], which is distinct from a [`MatchOutcome::Fail`] in that it
    /// represents an unexpected error as opposed to a matcher failing.
    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>>;

    /// The function called to test whether a value matches in the negative case.
    ///
    /// This is the same as [`match_pos`], except this is for when a matcher is negated (we're
    /// expecting it to fail).
    ///
    /// [`match_pos`]: crate::core::Match::match_pos
    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>>;
}

/// A simplified version of the [`Match`] trait for implementing matchers.
///
/// Implementing this trait is a simpler alternative when the matcher does not need to transform
/// values like the [`be_ok`] and [`be_some`] matchers do.
///
/// [`be_ok`]: crate::be_ok
/// [`be_some`]: crate::be_some
pub trait SimpleMatch<Actual> {
    /// The failure output that is passed to the formatter.
    ///
    /// This type serves the same purpose as [`Match::PosFail`] and [`Match::NegFail`], except for
    /// matchers that are implemented with [`SimpleMatch`], they are always the same type.
    type Fail;

    /// Returns `true` if the matcher succeeded or `false` if it failed.
    fn matches(&mut self, actual: &Actual) -> crate::Result<bool>;

    /// Consumes the "actual" value (the value passed to [`expect!`]) and results a [`Self::Fail`]
    /// that describes why the matcher failed.
    ///
    /// This will only ever be called if [`matches`] returns `false`.
    ///
    /// [`expect!`]: crate::expect
    fn fail(self, actual: Actual) -> Self::Fail;
}

/// An object-safe version of [`Match`].
///
/// This type is used internally and you should never have to implement it yourself.
pub trait DynMatch {
    /// Same as [`Match::In`].
    type In;

    /// Same as [`Match::PosOut`].
    type PosOut;

    /// Same as [`Match::NegOut`].
    type NegOut;

    /// An object-safe version of [`Match::match_pos`].
    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>>;

    /// An object-safe version of [`Match::match_neg`].
    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>>;
}

/// A boxed [`DynMatch`].
pub type BoxMatch<'a, In, PosOut, NegOut = PosOut> =
    Box<dyn DynMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a>;

/// A matcher.
///
/// This type is a matcher that can be used to make assertions. You can create a matcher from any
/// type which implements [`Match`] or [`SimpleMatch`].
pub struct Matcher<'a, In, PosOut, NegOut = PosOut> {
    inner: BoxMatch<'a, In, PosOut, NegOut>,
}

impl<'a, In, PosOut, NegOut> fmt::Debug for Matcher<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matcher").finish_non_exhaustive()
    }
}

impl<'a, In, PosOut, NegOut> Matcher<'a, In, PosOut, NegOut> {
    /// Create a new [`Matcher`] from a type that implements [`Match`] and a formatter.
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: Match<In = In, PosOut = PosOut, NegOut = NegOut> + 'a,
        Fmt: ResultFormat<Pos = M::PosFail, Neg = M::NegFail> + 'a,
    {
        Self {
            inner: Box::new(DynMatchAdapter::new(matcher, format)),
        }
    }

    /// Same as [`new`], but negates the matcher.
    ///
    /// If you want to create a negated version of a matcher, such as a counterpart of [`equal`]
    /// called `not_equal`, you can use this method to do so.
    ///
    /// [`new`]: crate::core::Matcher::new
    /// [`equal`]: crate::equal
    pub fn neg<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: Match<In = In, PosOut = NegOut, NegOut = PosOut> + 'a,
        Fmt: ResultFormat<Pos = M::NegFail, Neg = M::PosFail> + 'a,
    {
        Matcher::new(NegMatchAdapter::new(matcher), format)
    }

    /// Wrap this matcher with a new formatter.
    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        PosOut: 'a,
        NegOut: 'a,
        Fmt: ResultFormat<Pos = FormattedFailure, Neg = FormattedFailure> + 'a,
    {
        Self::new(MatchWrapper::new(self), format)
    }

    /// Convert this matcher into a [`BoxMatch`].
    pub fn into_box(self) -> BoxMatch<'a, In, PosOut, NegOut> {
        self.inner
    }
}

impl<'a, Actual> Matcher<'a, Actual, Actual> {
    /// Create a new [`Matcher`] from a type that implements [`SimpleMatch`] and a formatter.
    pub fn simple<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: SimpleMatch<Actual> + 'a,
        Fmt: ResultFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
        Actual: 'a,
    {
        Self::new(SimpleMatchAdapter::new(matcher), format)
    }

    /// Same as [`simple`], but negates the matcher.
    ///
    /// See [`neg`].
    ///
    /// [`simple`]: crate::core::Matcher::simple
    /// [`neg`]: crate::core::Matcher::neg
    pub fn simple_neg<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: SimpleMatch<Actual> + 'a,
        Fmt: ResultFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
        Actual: 'a,
    {
        Self::neg(SimpleMatchAdapter::new(matcher), format)
    }
}

impl<'a, In, PosOut, NegOut> DynMatch for Matcher<'a, In, PosOut, NegOut> {
    type In = In;

    type PosOut = PosOut;
    type NegOut = NegOut;

    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>> {
        self.inner.match_pos(actual)
    }

    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>> {
        self.inner.match_neg(actual)
    }
}
