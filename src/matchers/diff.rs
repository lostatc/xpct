#![cfg(feature = "diff")]

use std::borrow::{Borrow, Cow};
use std::fmt;
use std::hash::Hash;

use similar::{capture_diff_slices, ChangeTag, TextDiff};

const DIFF_ALGORITHM: similar::Algorithm = similar::Algorithm::Patience;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffTag {
    Insert,
    Delete,
    Equal,
}

impl DiffTag {
    fn from_similar(tag: ChangeTag) -> Self {
        match tag {
            ChangeTag::Equal => Self::Equal,
            ChangeTag::Delete => Self::Delete,
            ChangeTag::Insert => Self::Insert,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct DiffSegment<'a, T>
where
    T: ?Sized + ToOwned,
{
    pub value: Cow<'a, T>,
    pub tag: DiffTag,
}

impl<'a, T> fmt::Debug for DiffSegment<'a, T>
where
    T: ?Sized + ToOwned + fmt::Debug,
    T::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiffSegment")
            .field("value", &self.value)
            .field("tag", &self.tag)
            .finish()
    }
}

pub type Diff<'a, T> = Vec<DiffSegment<'a, T>>;

pub trait Diffable<'a> {
    type Other: ?Sized;

    type Segment: ?Sized + ToOwned;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
    where
        Q: Borrow<Self::Other>;
}

impl<'a> Diffable<'a> for str {
    type Other = str;

    type Segment = str;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
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
            .map(|change| DiffSegment {
                value: change.to_string_lossy(),
                tag: match change.tag() {
                    ChangeTag::Insert => DiffTag::Insert,
                    ChangeTag::Delete => DiffTag::Delete,
                    ChangeTag::Equal => DiffTag::Equal,
                },
            })
            .collect()
    }
}

impl<'a> Diffable<'a> for String {
    type Other = str;
    type Segment = str;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_str().diff(other)
    }
}

impl<'a> Diffable<'a> for Cow<'a, str> {
    type Other = str;
    type Segment = str;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_ref().diff(other)
    }
}

impl<'a, T> Diffable<'a> for [T]
where
    T: Clone + Hash + Ord,
{
    type Other = [T];
    type Segment = T;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        capture_diff_slices(DIFF_ALGORITHM, self, other.borrow())
            .into_iter()
            .flat_map(|op| op.iter_changes(self, other.borrow()))
            .map(|change| DiffSegment {
                value: Cow::Owned(change.value()),
                tag: DiffTag::from_similar(change.tag()),
            })
            .collect()
    }
}

impl<'a, T> Diffable<'a> for Vec<T>
where
    T: Clone + Hash + Ord,
{
    type Other = [T];
    type Segment = T;

    fn diff<Q>(&'a self, other: &'a Q) -> Diff<'a, Self::Segment>
    where
        Q: Borrow<Self::Other>,
    {
        self.as_slice().diff(other)
    }
}
