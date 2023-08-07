#![cfg(feature = "diff")]

use std::hash::Hash;

use similar::ChangeTag;

use crate::core::Match;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffKind {
    String,
    Slice,
    Set,
    Map,
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
/// The "something" that was added, removed, or unchanged in the diff is [`value`].
///
/// A diff segment is generic over its `Value`, which maps to [`Diffable::Segment`].
///
/// See [`Diffable`] for more information.
///
/// [`value`]: crate::matchers::DiffSegment::value
/// [`Diffable::Segment`]: crate::matchers::Diffable::Segment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiffSegment<Value> {
    /// The value of this segment.
    pub value: Value,

    /// Whether this segment represents an insertion, a deletion, or no change.
    pub tag: DiffTag,
}

/// A diff between an actual and expected value.
///
/// You can generate a [`Diff`] from any type which implements [`Diffable`].
pub type Diff<Segment> = Vec<DiffSegment<Segment>>;

/// A value which can be diffed against another value.
///
/// Diffing two values produces a [`Diff`], which consists of a list of [`DiffSegment`]s.
pub trait Diffable<Other> {
    /// The unit that diffs are broken up into.
    ///
    /// A diff consists of a list of segments that each represent an addition, a deletion, or no
    /// change. This type represents the segments that the diffable values should be broken up into.
    ///
    /// If you're diffing `Vec<T>`, the `Segment` could be `T`. If you're diffing strings, the
    /// `Segment` could be `char` or `str`.
    type Segment;

    const KIND: DiffKind;

    /// Generate a diff of this value and `other`.
    fn diff(&self, other: Other) -> Diff<Self::Segment>;

    /// The string representation of the value to use in the diff output.
    ///
    /// In practice, this will probably delegate to the type's [`Debug`] or [`Display`] impl.
    ///
    /// [`Debug`]: std::fmt::Debug
    /// [`Display`]: std::fmt::Display
    fn repr(segment: &Self::Segment) -> String;
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
    type Fail = Diff<Expected::Segment>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        self.expected.diff(actual)
    }
}
