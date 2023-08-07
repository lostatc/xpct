#![cfg(feature = "diff")]

use std::fmt;
use std::hash::Hash;

use similar::ChangeTag;

use crate::core::Match;

/// A discriminant that represents how a diff should be formatted.
///
/// You would format a diff of two strings differently from a diff of two slices. Diff "kinds" exist
/// to pass this information along to the [formatter][crate::core::Format].
///
/// The [`Custom`] variant exists for when you're implementing [`Diffable`] on your own types and
/// the none of the provided kinds are suitable. Note that the [provided
/// formatter][crate::format::DiffFormat] won't know what to do with custom diff kinds, so you would
/// need to implement your own formatter in this case.
///
/// [`Custom`]: crate::matchers::DiffKind::Custom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiffKind {
    /// Diffing strings.
    String,

    /// Diffing slices.
    Slice,

    /// Diffing sets.
    Set,

    /// Diffing maps.
    Map,

    /// Provide your own custom diff kind.
    Custom(&'static str),
}

/// Whether a [`DiffSegment`] represents an insertion, deletion, or no change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffTag {
    /// The segment is in the actual value but not the expected value.
    Insert,

    /// The segment is in the expected value but not the actual value.
    Delete,

    /// The segment is the same in both the actual and expected values.
    Equal,
}

impl DiffTag {
    pub(super) fn from_tag(tag: ChangeTag) -> Self {
        match tag {
            ChangeTag::Equal => Self::Equal,
            ChangeTag::Delete => Self::Delete,
            ChangeTag::Insert => Self::Insert,
        }
    }
}

/// A contiguous span in a diff.
///
/// A [`Diff`] consists of a list of segments. Each segment represents either:
///
/// 1. Something that is in the actual value but not the expected value (an insertion).
/// 2. Something that is in the expected value but not the actual value (a deletion).
/// 3. Something that is the same between the two values.
///
/// The "something" that was inserted, deleted, or unchanged in the diff is [`value`].
///
/// See [`Diffable`] for more information.
///
/// [`value`]: crate::matchers::DiffSegment::value
/// [`Diffable::Segment`]: crate::matchers::Diffable::Segment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiffSegment {
    /// The string representation of the thing that was inserted, deleted, or unchanged.
    pub value: String,

    /// Whether this segment represents an insertion, a deletion, or no change.
    pub tag: DiffTag,
}

impl DiffSegment {
    /// Create a [`DiffSegment`] from a value that implements [`Debug`].
    ///
    /// [`Debug`]: std::fmt::Debug
    pub fn from_debug(value: impl fmt::Debug, tag: DiffTag) -> Self {
        Self {
            value: format!("{:?}", value),
            tag,
        }
    }

    /// Create a [`DiffSegment`] from a value that implements [`Display`].
    ///
    /// [`Display`]: std::fmt::Display
    pub fn from_display(value: impl fmt::Display, tag: DiffTag) -> Self {
        Self {
            value: format!("{}", value),
            tag,
        }
    }
}

/// A diff between an actual and expected value.
///
/// You can generate a [`Diff`] from any type which implements [`Diffable`].
pub type Diff = Vec<DiffSegment>;

/// A value which can be diffed against another value.
///
/// Diffing two values produces a [`Diff`], which consists of a list of [`DiffSegment`]s.
pub trait Diffable<Other> {
    /// A discriminant that represents how the diff should be formatted.
    ///
    /// See [`DiffKind`].
    const KIND: DiffKind;

    /// Generate a diff of this value and `other`.
    fn diff(&self, other: Other) -> Diff;
}

/// The matcher for [`eq_diff`].
///
/// [`eq_diff`]: crate::eq_diff
#[derive(Debug)]
pub struct EqDiffMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqDiffMatcher<Expected> {
    /// Create a new [`EqDiffMatcher`] from the expected value.
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> Match<Actual> for EqDiffMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq,
    Expected: Diffable<Actual>,
{
    type Fail = Diff;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        self.expected.diff(actual)
    }
}
