#![cfg(feature = "diff")]

use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;

use similar::utils::TextDiffRemapper;
use similar::{capture_diff_slices, ChangeTag, TextDiff};

use crate::core::Match;

const DIFF_ALGORITHM: similar::Algorithm = similar::Algorithm::Patience;

/// A [`Diffable::KIND`] for strings.
///
/// [`Diffable::KIND`]: crate::matchers::Diffable::KIND
pub const STRING_DIFF_KIND: &str = "string";

/// A [`Diffable::KIND`] for slices.
///
/// [`Diffable::KIND`]: crate::matchers::Diffable::KIND
pub const SLICE_DIFF_KIND: &str = "slice";

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
    fn from_tag(tag: ChangeTag) -> Self {
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
/// The "something" that was added, removed, or unchanged in the diff is returned by [`value`].
///
/// A diff segment is generic over its `Value`, which maps to [`Diffable::Segment`].
///
/// See [`Diffable`] for more information.
///
/// [`value`]: crate::matchers::DiffSegment::value
/// [`Diffable::Segment`]: crate::matchers::Diffable::Segment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegment<Value> {
    value: Value,
    tag: DiffTag,
}

impl<Value> DiffSegment<Value> {
    /// Create a new [`DiffSegment`] from a value and a tag.
    pub fn new(value: Value, tag: DiffTag) -> Self {
        Self { value, tag }
    }

    /// Return the value of this segment.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Return whether this segment represents an insertion, a deletion, or no change.
    pub fn tag(&self) -> DiffTag {
        self.tag
    }
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

    /// A discriminant that represents how this diffable should be formatted.
    ///
    /// This value allows [formatters][crate::core::Format] to represent different kinds of diffable
    /// values differently. For example, a diff of two strings should be formatted differently from
    /// a diff of two vectors of strings, even if they both have a [`Segment`] of type `String`.
    ///
    /// [`STRING_DIFF_KIND`] and [`SLICE_DIFF_KIND`] are two examples of provided diff kinds.
    ///
    /// [`Segment`]: crate::matchers::Diffable::Segment
    const KIND: &'static str;

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

impl<'a> Diffable<&'a str> for &'a str {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: &'a str) -> Diff<Self::Segment> {
        #[cfg(feature = "unicode-diff")]
        let text_diff = TextDiff::configure()
            .algorithm(DIFF_ALGORITHM)
            .diff_graphemes(*self, other);

        #[cfg(not(feature = "unicode-diff"))]
        let text_diff = TextDiff::configure()
            .algorithm(DIFF_ALGORITHM)
            .diff_chars(self, other);

        let remapper = TextDiffRemapper::from_text_diff(&text_diff, self, other.borrow());

        text_diff
            .ops()
            .iter()
            .flat_map(move |op| remapper.iter_slices(op))
            .map(|(tag, slice)| DiffSegment::new(slice.to_owned(), DiffTag::from_tag(tag)))
            .collect()
    }

    fn repr(segment: &Self::Segment) -> String {
        segment.to_string()
    }
}

impl<'a, T> Diffable<&'a [T]> for &'a [T]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &'a [T]) -> Diff<Self::Segment> {
        capture_diff_slices(DIFF_ALGORITHM, self, other)
            .into_iter()
            .flat_map(|op| op.iter_changes(*self, other))
            .map(|change| DiffSegment::new(change.value(), DiffTag::from_tag(change.tag())))
            .collect()
    }

    fn repr(segment: &Self::Segment) -> String {
        format!("{:?}", segment)
    }
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
