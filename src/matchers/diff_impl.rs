#![cfg(feature = "diff")]

use std::borrow::Cow;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use similar::{capture_diff_slices, utils::TextDiffRemapper, TextDiff};

use super::diff::SET_DIFF_KIND;
use super::{
    Diff, DiffSegment, DiffTag, Diffable, MAP_DIFF_KIND, SLICE_DIFF_KIND, STRING_DIFF_KIND,
};

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

        let remapper = TextDiffRemapper::from_text_diff(&text_diff, self, other);

        text_diff
            .ops()
            .iter()
            .flat_map(move |op| remapper.iter_slices(op))
            .map(|(tag, slice)| DiffSegment {
                value: slice.to_owned(),
                tag: DiffTag::from_tag(tag),
            })
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
            .map(|change| DiffSegment {
                value: change.value(),
                tag: DiffTag::from_tag(change.tag()),
            })
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

impl<T> Diffable<&HashSet<T>> for &HashSet<T>
where
    T: Eq + Hash + Clone + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SET_DIFF_KIND;

    fn diff(&self, other: &HashSet<T>) -> Diff<Self::Segment> {
        let deletions = self.difference(other).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Delete,
        });

        let equal = self.intersection(other).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Equal,
        });

        let insertions = other.difference(self).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Insert,
        });

        let mut segments = deletions.collect::<Vec<_>>();
        segments.extend(equal);
        segments.extend(insertions);

        segments
    }

    fn repr(segment: &Self::Segment) -> String {
        format!("{:?}", segment)
    }
}

impl<T> Diffable<HashSet<T>> for HashSet<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SET_DIFF_KIND;

    fn diff(&self, other: HashSet<T>) -> Diff<Self::Segment> {
        <&HashSet<T>>::diff(&self, &other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&HashSet<T>>::repr(segment)
    }
}

impl<T> Diffable<&BTreeSet<T>> for &BTreeSet<T>
where
    T: Eq + Hash + Ord + Clone + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SET_DIFF_KIND;

    fn diff(&self, other: &BTreeSet<T>) -> Diff<Self::Segment> {
        let deletions = self.difference(other).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Delete,
        });

        let equal = self.intersection(other).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Equal,
        });

        let insertions = other.difference(self).map(|element| DiffSegment {
            value: element.to_owned(),
            tag: DiffTag::Insert,
        });

        let mut segments = deletions.collect::<Vec<_>>();
        segments.extend(equal);
        segments.extend(insertions);

        segments
    }

    fn repr(segment: &Self::Segment) -> String {
        format!("{:?}", segment)
    }
}

impl<T> Diffable<BTreeSet<T>> for BTreeSet<T>
where
    T: Clone + Hash + Ord + fmt::Debug,
{
    type Segment = T;

    const KIND: &'static str = SET_DIFF_KIND;

    fn diff(&self, other: BTreeSet<T>) -> Diff<Self::Segment> {
        <&BTreeSet<T>>::diff(&self, &other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&BTreeSet<T>>::repr(segment)
    }
}

/// Returns pairs in `this` but not `other`.
fn hash_map_difference<K, V>(this: &HashMap<K, V>, other: &HashMap<K, V>) -> Vec<(K, V)>
where
    K: Eq + Hash + Clone,
    V: Eq + Clone,
{
    let mut pairs = Vec::with_capacity(this.len());

    for (this_key, this_value) in this {
        let pair = match other.get(this_key) {
            Some(other_value) if this_value != other_value => {
                (this_key.to_owned(), this_value.to_owned())
            }
            None => (this_key.to_owned(), this_value.to_owned()),
            _ => continue,
        };

        pairs.push(pair);
    }

    pairs.shrink_to_fit();

    pairs
}

impl<K, V> Diffable<&HashMap<K, V>> for &HashMap<K, V>
where
    K: Eq + Hash + Clone + fmt::Debug,
    V: Eq + Clone + fmt::Debug,
{
    type Segment = (K, V);

    const KIND: &'static str = MAP_DIFF_KIND;

    fn diff(&self, other: &HashMap<K, V>) -> Diff<Self::Segment> {
        let mut segments = Vec::with_capacity(self.len() + other.len());

        // Pairs in `self` but not `other`.
        let deletions = hash_map_difference(self, other)
            .into_iter()
            .map(|pair| DiffSegment {
                value: pair,
                tag: DiffTag::Delete,
            });

        segments.extend(deletions);

        // Pairs in both `self` and `other`.
        let mut equal = Vec::with_capacity(cmp::max(self.len(), other.len()));

        for (this_key, this_value) in *self {
            let pair = match other.get(this_key) {
                Some(other_value) if this_value == other_value => {
                    (this_key.to_owned(), other_value.to_owned())
                }
                _ => continue,
            };

            equal.push(DiffSegment {
                value: pair,
                tag: DiffTag::Equal,
            });
        }

        segments.extend(equal);

        // Pairs in `other` but not `self`.
        let insertions = hash_map_difference(other, self)
            .into_iter()
            .map(|pair| DiffSegment {
                value: pair,
                tag: DiffTag::Insert,
            });

        segments.extend(insertions);

        segments.shrink_to_fit();

        segments
    }

    fn repr(segment: &Self::Segment) -> String {
        format!("{:?}", segment)
    }
}

impl<K, V> Diffable<HashMap<K, V>> for HashMap<K, V>
where
    K: Eq + Hash + Clone + fmt::Debug,
    V: Eq + Clone + fmt::Debug,
{
    type Segment = (K, V);

    const KIND: &'static str = MAP_DIFF_KIND;

    fn diff(&self, other: HashMap<K, V>) -> Diff<Self::Segment> {
        <&HashMap<K, V>>::diff(&self, &other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&HashMap<K, V>>::repr(segment)
    }
}

/// Returns pairs in `this` but not `other`.
fn btree_map_difference<K, V>(this: &BTreeMap<K, V>, other: &BTreeMap<K, V>) -> Vec<(K, V)>
where
    K: Eq + Hash + Ord + Clone,
    V: Eq + Clone,
{
    let mut pairs = Vec::with_capacity(this.len());

    for (this_key, this_value) in this {
        let pair = match other.get(this_key) {
            Some(other_value) if this_value != other_value => {
                (this_key.to_owned(), this_value.to_owned())
            }
            None => (this_key.to_owned(), this_value.to_owned()),
            _ => continue,
        };

        pairs.push(pair);
    }

    pairs.shrink_to_fit();

    pairs
}

impl<K, V> Diffable<&BTreeMap<K, V>> for &BTreeMap<K, V>
where
    K: Eq + Hash + Ord + Clone + fmt::Debug,
    V: Eq + Clone + fmt::Debug,
{
    type Segment = (K, V);

    const KIND: &'static str = MAP_DIFF_KIND;

    fn diff(&self, other: &BTreeMap<K, V>) -> Diff<Self::Segment> {
        let mut segments = Vec::with_capacity(self.len() + other.len());

        // Pairs in `self` but not `other`.
        let deletions = btree_map_difference(self, other)
            .into_iter()
            .map(|pair| DiffSegment {
                value: pair,
                tag: DiffTag::Delete,
            });

        segments.extend(deletions);

        // Pairs in both `self` and `other`.
        let mut equal = Vec::with_capacity(cmp::max(self.len(), other.len()));

        for (this_key, this_value) in *self {
            let pair = match other.get(this_key) {
                Some(other_value) if this_value == other_value => {
                    (this_key.to_owned(), other_value.to_owned())
                }
                _ => continue,
            };

            equal.push(DiffSegment {
                value: pair,
                tag: DiffTag::Equal,
            });
        }

        segments.extend(equal);

        // Pairs in `other` but not `self`.
        let insertions = btree_map_difference(other, self)
            .into_iter()
            .map(|pair| DiffSegment {
                value: pair,
                tag: DiffTag::Insert,
            });

        segments.extend(insertions);

        segments.shrink_to_fit();

        segments
    }

    fn repr(segment: &Self::Segment) -> String {
        format!("{:?}", segment)
    }
}

impl<K, V> Diffable<BTreeMap<K, V>> for BTreeMap<K, V>
where
    K: Eq + Hash + Ord + Clone + fmt::Debug,
    V: Eq + Clone + fmt::Debug,
{
    type Segment = (K, V);

    const KIND: &'static str = MAP_DIFF_KIND;

    fn diff(&self, other: BTreeMap<K, V>) -> Diff<Self::Segment> {
        <&BTreeMap<K, V>>::diff(&self, &other)
    }

    fn repr(segment: &Self::Segment) -> String {
        <&BTreeMap<K, V>>::repr(segment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_identical_strings() {
        let actual = "foobar".diff("foobar");

        let expected = vec![DiffSegment {
            value: "foobar".to_string(),
            tag: DiffTag::Equal,
        }];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_strings_with_addition() {
        let actual = "foo".diff("foobar");

        let expected = vec![
            DiffSegment {
                value: "foo".to_string(),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "bar".to_string(),
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_strings_with_deletion() {
        let actual = "foobar".diff("foo");

        let expected = vec![
            DiffSegment {
                value: "foo".to_string(),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "bar".to_string(),
                tag: DiffTag::Delete,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_strings_with_addition_and_deletion() {
        let actual = "bar".diff("foobarbaz");

        let expected = vec![
            DiffSegment {
                value: "foo".to_string(),
                tag: DiffTag::Insert,
            },
            DiffSegment {
                value: "bar".to_string(),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "baz".to_string(),
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_identical_slices() {
        let actual = ["a", "b", "c"].diff(["a", "b", "c"]);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "c",
                tag: DiffTag::Equal,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_slices_with_addition() {
        let actual = ["a"].diff(["a", "b"]);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_slices_with_deletion() {
        let actual = ["a", "b"].diff(["a"]);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Delete,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_slices_with_addition_and_deletion() {
        let actual = ["a", "b"].diff(["b", "c"]);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Delete,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "c",
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    //
    // For sets and maps, we always order deleted elements before elements than are equal before
    // elements that were added.
    //

    #[test]
    fn diff_identical_sets() {
        let before = ["a", "b", "c"].into_iter().collect::<HashSet<_>>();
        let after = ["a", "b", "c"].into_iter().collect::<HashSet<_>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "c",
                tag: DiffTag::Equal,
            },
        ];

        assert_eq!(
            actual.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn diff_sets_with_addition() {
        let before = ["a"].into_iter().collect::<HashSet<_>>();
        let after = ["a", "b"].into_iter().collect::<HashSet<_>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_sets_with_deletion() {
        let before = ["a", "b"].into_iter().collect::<HashSet<_>>();
        let after = ["a"].into_iter().collect::<HashSet<_>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: "b",
                tag: DiffTag::Delete,
            },
            DiffSegment {
                value: "a",
                tag: DiffTag::Equal,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_sets_with_addition_and_deletion() {
        let before = ["a", "b"].into_iter().collect::<HashSet<_>>();
        let after = ["b", "c"].into_iter().collect::<HashSet<_>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: "a",
                tag: DiffTag::Delete,
            },
            DiffSegment {
                value: "b",
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: "c",
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_identical_maps() {
        let before = [("a", 1), ("b", 2)].into_iter().collect::<HashMap<_, _>>();
        let after = [("a", 1), ("b", 2)].into_iter().collect::<HashMap<_, _>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: ("a", 1),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: ("b", 2),
                tag: DiffTag::Equal,
            },
        ];

        assert_eq!(
            actual.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn diff_maps_with_addition() {
        let before = [("a", 1)].into_iter().collect::<HashMap<_, _>>();
        let after = [("a", 1), ("b", 2)].into_iter().collect::<HashMap<_, _>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: ("a", 1),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: ("b", 2),
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_maps_with_deletion() {
        let before = [("a", 1), ("b", 2)].into_iter().collect::<HashMap<_, _>>();
        let after = [("a", 1)].into_iter().collect::<HashMap<_, _>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: ("b", 2),
                tag: DiffTag::Delete,
            },
            DiffSegment {
                value: ("a", 1),
                tag: DiffTag::Equal,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn diff_maps_with_addition_and_deletion() {
        let before = [("a", 1), ("b", 2)].into_iter().collect::<HashMap<_, _>>();
        let after = [("b", 2), ("c", 3)].into_iter().collect::<HashMap<_, _>>();

        let actual = before.diff(after);

        let expected = vec![
            DiffSegment {
                value: ("a", 1),
                tag: DiffTag::Delete,
            },
            DiffSegment {
                value: ("b", 2),
                tag: DiffTag::Equal,
            },
            DiffSegment {
                value: ("c", 3),
                tag: DiffTag::Insert,
            },
        ];

        assert_eq!(actual, expected);
    }
}
