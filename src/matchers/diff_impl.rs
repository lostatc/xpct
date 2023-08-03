#![cfg(feature = "diff")]

use std::borrow::{Borrow, Cow};
use std::fmt;
use std::hash::Hash;

use similar::{capture_diff_slices, utils::TextDiffRemapper, TextDiff};

use super::{Diff, DiffSegment, DiffTag, Diffable, SLICE_DIFF_KIND, STRING_DIFF_KIND};

const DIFF_ALGORITHM: similar::Algorithm = similar::Algorithm::Patience;

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

impl<'a> Diffable<String> for &'a str {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: String) -> Diff<Self::Segment> {
        self.diff(other.as_str())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<Cow<'a, str>> for &'a str {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: Cow<'a, str>) -> Diff<Self::Segment> {
        self.diff(other.as_ref())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<&'a str> for String {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: &'a str) -> Diff<Self::Segment> {
        self.as_str().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<String> for String {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: String) -> Diff<Self::Segment> {
        self.as_str().diff(other.as_str())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<Cow<'a, str>> for String {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: Cow<'a, str>) -> Diff<Self::Segment> {
        self.as_str().diff(other.as_ref())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<&'a str> for Cow<'a, str> {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: &'a str) -> Diff<Self::Segment> {
        self.as_ref().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<String> for Cow<'a, str> {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: String) -> Diff<Self::Segment> {
        self.as_ref().diff(other.as_str())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
    }
}

impl<'a> Diffable<Cow<'a, str>> for Cow<'a, str> {
    type Segment = String;

    const KIND: &'static str = STRING_DIFF_KIND;

    fn diff(&self, other: Cow<'a, str>) -> Diff<Self::Segment> {
        self.as_ref().diff(other.as_ref())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&'a str as Diffable<&'a str>>::repr(segment)
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

impl<'a, T, const OTHER_LEN: usize> Diffable<[T; OTHER_LEN]> for &'a [T]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: [T; OTHER_LEN]) -> Diff<Self::Segment> {
        self.diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T, const OTHER_LEN: usize> Diffable<&[T; OTHER_LEN]> for &'a [T]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &[T; OTHER_LEN]) -> Diff<Self::Segment> {
        self.diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T> Diffable<Vec<T>> for &'a [T]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: Vec<T>) -> Diff<Self::Segment> {
        self.diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T> Diffable<&Vec<T>> for &'a [T]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &Vec<T>) -> Diff<Self::Segment> {
        self.diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T, const LEN: usize> Diffable<&'a [T]> for [T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &'a [T]) -> Diff<Self::Segment> {
        self.as_slice().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T, const LEN: usize, const OTHER_LEN: usize> Diffable<[T; OTHER_LEN]> for [T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: [T; OTHER_LEN]) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T, const LEN: usize> Diffable<Vec<T>> for [T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: Vec<T>) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T, const LEN: usize> Diffable<&'a [T]> for &[T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &'a [T]) -> Diff<Self::Segment> {
        self.as_slice().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T, const LEN: usize, const OTHER_LEN: usize> Diffable<&[T; OTHER_LEN]> for &[T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &[T; OTHER_LEN]) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T, const LEN: usize> Diffable<Vec<T>> for &[T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: Vec<T>) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T, const LEN: usize> Diffable<&Vec<T>> for &[T; LEN]
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &Vec<T>) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T> Diffable<&'a [T]> for Vec<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &'a [T]) -> Diff<Self::Segment> {
        self.as_slice().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T> Diffable<Vec<T>> for Vec<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: Vec<T>) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<'a, T> Diffable<&'a [T]> for &Vec<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &'a [T]) -> Diff<Self::Segment> {
        self.as_slice().diff(other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}

impl<T> Diffable<&Vec<T>> for &Vec<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SLICE_DIFF_KIND;

    fn diff(&self, other: &Vec<T>) -> Diff<Self::Segment> {
        self.as_slice().diff(other.as_slice())
    }

    fn repr(segment: &Self::Segment) -> String {
        <&[T] as Diffable<&[T]>>::repr(segment)
    }
}
