#![cfg(feature = "diff")]

use std::borrow::{Borrow, Cow};
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
pub trait Diffable {
    /// The value to be diffed against.
    type Other: ?Sized;

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
    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>;
}

impl Diffable for str {
    type Other = str;
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        #[cfg(feature = "unicode-diff")]
        let text_diff = TextDiff::configure()
            .algorithm(DIFF_ALGORITHM)
            .diff_graphemes(self, other.borrow());

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
}

impl Diffable for String {
    type Other = str;
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_str().diff(other)
    }
}

impl<'a> Diffable for Cow<'a, str> {
    type Other = str;
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_ref().diff(other)
    }
}

impl<T> Diffable for [T]
where
    T: Clone + Hash + Ord,
{
    type Other = [T];
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        capture_diff_slices(DIFF_ALGORITHM, self, other.borrow())
            .into_iter()
            .flat_map(|op| op.iter_changes(self, other.borrow()))
            .map(|change| DiffSegment::new(change.value(), DiffTag::from_tag(change.tag())))
            .collect()
    }
}

impl<T> Diffable for Vec<T>
where
    T: Clone + Hash + Ord,
{
    type Other = [T];
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_slice().diff(other)
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
    Expected: Diffable<Other = Actual>,
{
    type Fail = Diff<Expected::Segment>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        self.expected.diff(&actual)
    }
}
