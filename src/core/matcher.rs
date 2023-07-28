use std::fmt;

use super::adapter::{DynTransformMatchAdapter, NegTransformMatchAdapter, SimpleMatchAdapter};
use super::wrap::MatchWrapper;
use super::{FormattedFailure, MatchOutcome, MatcherFormat};

/// The trait which is implemented to create matchers that transform their values.
///
/// For simple matchers that don't need to transform their values, it may be easier to implement
/// [`SimpleMatch`] instead.
pub trait TransformMatch {
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
    /// This is the same as [`Self::PosOut`], except this is for when a matcher is negated (when
    /// we're expecting it to fail).
    type NegOut;

    /// The failure output that is passed to the formatter.
    ///
    /// This value describes the reason why the matcher failed in a presentation-agnostic way so
    /// that a formatter can use it to generate its own output format. Values like [`Mismatch`] and
    /// [`Expectation`] are meant to be used for this.
    ///
    /// [`Mismatch`]: crate::matchers::Mismatch
    /// [`Expectation`]: crate::matchers::Expectation
    type PosFail;

    /// The failure output of the matcher in the negative case.
    ///
    /// This is the same as [`Self::PosFail`], except this is for when a matcher is negated (when
    /// we're expecting it to fail).
    type NegFail;

    /// The function called to test whether a value matches.
    ///
    /// This returns a [`MatchOutcome`], which determines whether the matcher succeeded or failed.
    /// It can also return an `Err`, which is distinct from a [`MatchOutcome::Fail`] in that it
    /// represents an unexpected error as opposed to the matcher failing.
    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>>;

    /// The function called to test whether a value matches in the negative case.
    ///
    /// This is the same as [`match_pos`], except this is for when a matcher is negated (when we're
    /// expecting it to fail).
    ///
    /// [`match_pos`]: crate::core::TransformMatch::match_pos
    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>>;
}

/// A simplified version of the [`TransformMatch`] trait for implementing matchers.
///
/// Implementing this trait is a simpler alternative when the matcher does not need to transform
/// values like the [`be_ok`] and [`be_some`] matchers do.
///
/// [`be_ok`]: crate::be_ok
/// [`be_some`]: crate::be_some
pub trait SimpleMatch<Actual> {
    /// The failure output that is passed to the formatter.
    ///
    /// This type serves the same purpose as [`TransformMatch::PosFail`] and [`TransformMatch::NegFail`], except for
    /// matchers that are implemented with [`SimpleMatch`], they are always the same type.
    type Fail;

    /// Returns `true` if the matcher succeeded or `false` if it failed.
    ///
    /// This can also return an `Err`, which is distinct from returning `Ok(false)` in that it
    /// represents an unexpected error as opposed to the matcher failing.
    fn matches(&mut self, actual: &Actual) -> crate::Result<bool>;

    /// Consumes the "actual" value (the value passed to [`expect!`]) and returns a [`Self::Fail`]
    /// that describes why the matcher failed.
    ///
    /// This will only ever be called if [`matches`] returns `false`.
    ///
    /// [`expect!`]: crate::expect
    /// [`matches`]: crate::core::SimpleMatch::matches
    fn fail(self, actual: Actual) -> Self::Fail;
}

/// An object-safe version of [`TransformMatch`].
///
/// This type replaces the [`PosFail`] and [`NegFail`] associated types of [`TransformMatch`] with
/// [`FormattedFailure`] values.
///
/// This type is used internally and you should never have to implement it yourself.
///
/// [`PosFail`]: crate::core::TransformMatch::PosFail
/// [`NegFail`]: crate::core::TransformMatch::NegFail
pub trait DynTransformMatch {
    /// Same as [`TransformMatch::In`].
    type In;

    /// Same as [`TransformMatch::PosOut`].
    type PosOut;

    /// Same as [`TransformMatch::NegOut`].
    type NegOut;

    /// An object-safe version of [`TransformMatch::match_pos`].
    fn match_pos(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, FormattedFailure>>;

    /// An object-safe version of [`TransformMatch::match_neg`].
    fn match_neg(
        self: Box<Self>,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, FormattedFailure>>;
}

/// A boxed [`DynTransformMatch`].
pub type BoxTransformMatch<'a, In, PosOut, NegOut = PosOut> =
    Box<dyn DynTransformMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a>;

/// A matcher.
///
/// This type is a matcher that can be used to make assertions. You can create a matcher from any
/// type which implements [`TransformMatch`] or [`SimpleMatch`].
pub struct Matcher<'a, In, PosOut, NegOut = PosOut> {
    inner: BoxTransformMatch<'a, In, PosOut, NegOut>,
}

impl<'a, In, PosOut, NegOut> fmt::Debug for Matcher<'a, In, PosOut, NegOut> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matcher").finish_non_exhaustive()
    }
}

impl<'a, In, PosOut, NegOut> Matcher<'a, In, PosOut, NegOut> {
    /// Create a new [`Matcher`] from a type that implements [`TransformMatch`] and a formatter.
    pub fn new<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: TransformMatch<In = In, PosOut = PosOut, NegOut = NegOut> + 'a,
        Fmt: MatcherFormat<Pos = M::PosFail, Neg = M::NegFail> + 'a,
    {
        Self {
            inner: Box::new(DynTransformMatchAdapter::new(matcher, format)),
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
        M: TransformMatch<In = In, PosOut = NegOut, NegOut = PosOut> + 'a,
        Fmt: MatcherFormat<Pos = M::NegFail, Neg = M::PosFail> + 'a,
    {
        Matcher::new(NegTransformMatchAdapter::new(matcher), format)
    }

    /// Wrap this matcher with a new formatter.
    pub fn wrapped<Fmt>(self, format: Fmt) -> Self
    where
        In: 'a,
        PosOut: 'a,
        NegOut: 'a,
        Fmt: MatcherFormat<Pos = FormattedFailure, Neg = FormattedFailure> + 'a,
    {
        Self::new(MatchWrapper::new(self), format)
    }

    /// Convert this matcher into a [`BoxTransformMatch`].
    pub fn into_box(self) -> BoxTransformMatch<'a, In, PosOut, NegOut> {
        self.inner
    }
}

impl<'a, Actual> Matcher<'a, Actual, Actual> {
    /// Create a new [`Matcher`] from a type that implements [`SimpleMatch`] and a formatter.
    pub fn simple<M, Fmt>(matcher: M, format: Fmt) -> Self
    where
        M: SimpleMatch<Actual> + 'a,
        Fmt: MatcherFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
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
        Fmt: MatcherFormat<Pos = M::Fail, Neg = M::Fail> + 'a,
        Actual: 'a,
    {
        Self::neg(SimpleMatchAdapter::new(matcher), format)
    }
}

impl<'a, In, PosOut, NegOut> DynTransformMatch for Matcher<'a, In, PosOut, NegOut> {
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
