#![cfg(feature = "diff")]

use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::marker::PhantomData;

use similar::{capture_diff_slices, ChangeTag, TextDiff};

use crate::core::Match;

const DIFF_ALGORITHM: similar::Algorithm = similar::Algorithm::Patience;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffTag {
    Insert,
    Delete,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegment<Diffable: ?Sized, Segment> {
    value: Segment,
    tag: DiffTag,
    marker: PhantomData<Diffable>,
}

impl<Diffable: ?Sized, Segment> DiffSegment<Diffable, Segment> {
    pub fn new(value: Segment, kind: DiffTag) -> Self {
        Self {
            value,
            tag: kind,
            marker: PhantomData,
        }
    }

    pub fn value(&self) -> &Segment {
        &self.value
    }

    pub fn tag(&self) -> DiffTag {
        self.tag
    }
}

pub type Diff<Diffable, Segment> = Vec<DiffSegment<Diffable, Segment>>;

pub trait Diffable {
    type Other: ?Sized;
    type Segment;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
    where
        Q: Borrow<Self::Other>;
}

impl Diffable for str {
    type Other = str;
    type Segment = String;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
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

        text_diff
            .iter_all_changes()
            .map(|change| {
                DiffSegment::new(
                    change.to_string_lossy().into_owned(),
                    match change.tag() {
                        ChangeTag::Insert => DiffTag::Insert,
                        ChangeTag::Delete => DiffTag::Delete,
                        ChangeTag::Equal => DiffTag::Equal,
                    },
                )
            })
            .collect()
    }
}

impl Diffable for String {
    type Other = str;
    type Segment = String;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_str().diff(other)
    }
}

impl<'a> Diffable for Cow<'a, str> {
    type Other = str;
    type Segment = String;

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
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

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
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

    fn diff<Q>(&self, other: &Q) -> Diff<Self::Other, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_slice().diff(other)
    }
}

/// The matcher for [`diff_eq`].
///
/// [`diff_eq`]: crate::diff_eq
#[derive(Debug)]
pub struct DiffEqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> DiffEqualMatcher<Expected> {
    /// Create a new [`DiffEqualMatcher`] from the expected value.
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> Match<Actual> for DiffEqualMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq + Diffable<Other = Expected>,
{
    type Fail = Diff<Expected, Actual::Segment>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        actual.diff(&self.expected)
    }
}
