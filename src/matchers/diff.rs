#![cfg(feature = "diff")]

use std::borrow::Cow;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegment<'a, T>
where
    T: ?Sized + ToOwned,
    T::Owned: fmt::Debug,
{
    pub value: Cow<'a, T>,
    pub tag: DiffTag,
}

pub trait Diffable<'a> {
    type Segment: ?Sized + ToOwned;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<'a, Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug;
}

impl<'a> Diffable<'a> for str {
    type Segment = str;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug,
    {
        #[cfg(feature = "unicode-diff")]
        let text_diff = TextDiff::configure()
            .algorithm(DIFF_ALGORITHM)
            .diff_graphemes(self, other);

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
    type Segment = str;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<'a, Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug,
    {
        self.as_str().diff(other)
    }
}

impl<'a> Diffable<'a> for Cow<'a, str> {
    type Segment = str;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<'a, Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug,
    {
        self.as_ref().diff(other)
    }
}

impl<'a, T> Diffable<'a> for [T]
where
    T: Clone + Hash + Ord,
{
    type Segment = T;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<'a, Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug,
    {
        capture_diff_slices(DIFF_ALGORITHM, self, other)
            .into_iter()
            .flat_map(|op| op.iter_changes(self, other))
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
    type Segment = T;

    fn diff(&'a self, other: &'a Self) -> Vec<DiffSegment<'a, Self::Segment>>
    where
        <Self::Segment as ToOwned>::Owned: fmt::Debug,
    {
        self.as_slice().diff(other)
    }
}
