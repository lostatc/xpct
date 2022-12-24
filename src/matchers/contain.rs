use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use crate::core::SimpleMatch;

use super::{Len, Mismatch};

/// A collection that supports testing for membership.
pub trait Contains<T: ?Sized> {
    /// The collection contains the given element.
    fn contains(&self, element: &T) -> bool;
}

impl<T, const N: usize> Contains<T> for [T; N]
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        <[T]>::contains(self, element)
    }
}

impl<T, const N: usize> Contains<T> for &[T; N]
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        self.as_slice().contains(element)
    }
}

impl<T> Contains<T> for &[T]
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        <[T]>::contains(self, element)
    }
}

impl<T> Contains<T> for Vec<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        AsRef::<[T]>::as_ref(self).contains(element)
    }
}

impl<T> Contains<T> for &Vec<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        AsRef::<[T]>::as_ref(self).contains(element)
    }
}

impl<T> Contains<T> for LinkedList<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        LinkedList::contains(self, element)
    }
}

impl<T> Contains<T> for &LinkedList<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        LinkedList::contains(self, element)
    }
}

impl<T> Contains<T> for VecDeque<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        VecDeque::contains(self, element)
    }
}

impl<T> Contains<T> for &VecDeque<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        VecDeque::contains(self, element)
    }
}

impl<'a> Contains<str> for &'a str {
    fn contains(&self, element: &str) -> bool {
        <str>::contains(self, element)
    }
}

impl<'a> Contains<char> for &'a str {
    fn contains(&self, element: &char) -> bool {
        <str>::contains(self, *element)
    }
}

impl<'a> Contains<[char]> for &'a str {
    fn contains(&self, element: &[char]) -> bool {
        <str>::contains(self, element)
    }
}

impl Contains<str> for String {
    fn contains(&self, element: &str) -> bool {
        self.as_str().contains(element)
    }
}

impl Contains<char> for String {
    fn contains(&self, element: &char) -> bool {
        self.as_str().contains(*element)
    }
}

impl Contains<[char]> for String {
    fn contains(&self, element: &[char]) -> bool {
        self.as_str().contains(element)
    }
}

impl Contains<str> for &String {
    fn contains(&self, element: &str) -> bool {
        self.as_str().contains(element)
    }
}

impl Contains<char> for &String {
    fn contains(&self, element: &char) -> bool {
        self.as_str().contains(*element)
    }
}

impl Contains<[char]> for &String {
    fn contains(&self, element: &[char]) -> bool {
        self.as_str().contains(element)
    }
}

impl<'a> Contains<str> for Cow<'a, str> {
    fn contains(&self, element: &str) -> bool {
        self.as_ref().contains(element)
    }
}

impl<'a> Contains<char> for Cow<'a, str> {
    fn contains(&self, element: &char) -> bool {
        self.as_ref().contains(*element)
    }
}

impl<'a> Contains<[char]> for Cow<'a, str> {
    fn contains(&self, element: &[char]) -> bool {
        self.as_ref().contains(element)
    }
}

impl<'a> Contains<str> for &Cow<'a, str> {
    fn contains(&self, element: &str) -> bool {
        self.as_ref().contains(element)
    }
}

impl<'a> Contains<char> for &Cow<'a, str> {
    fn contains(&self, element: &char) -> bool {
        self.as_ref().contains(*element)
    }
}

impl<'a> Contains<[char]> for &Cow<'a, str> {
    fn contains(&self, element: &[char]) -> bool {
        self.as_ref().contains(element)
    }
}

impl<T> Contains<T> for HashSet<T>
where
    T: Hash + PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        HashSet::contains(self, element)
    }
}

impl<T> Contains<T> for &HashSet<T>
where
    T: Hash + PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        HashSet::contains(self, element)
    }
}

impl<T> Contains<T> for BTreeSet<T>
where
    T: Ord,
{
    fn contains(&self, element: &T) -> bool {
        BTreeSet::contains(self, element)
    }
}

impl<T> Contains<T> for &BTreeSet<T>
where
    T: Ord,
{
    fn contains(&self, element: &T) -> bool {
        BTreeSet::contains(self, element)
    }
}

impl<K, V> Contains<K> for HashMap<K, V>
where
    K: Hash + PartialEq<K> + Eq,
{
    fn contains(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}

impl<K, V> Contains<K> for &HashMap<K, V>
where
    K: Hash + PartialEq<K> + Eq,
{
    fn contains(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}

impl<K, V> Contains<K> for BTreeMap<K, V>
where
    K: Ord,
{
    fn contains(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}

impl<K, V> Contains<K> for &BTreeMap<K, V>
where
    K: Ord,
{
    fn contains(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}

impl<T> Contains<T> for Range<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        Range::contains(self, element)
    }
}

impl<T> Contains<T> for &Range<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        Range::contains(self, element)
    }
}

impl<T> Contains<T> for RangeFrom<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeFrom::contains(self, element)
    }
}

impl<T> Contains<T> for &RangeFrom<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeFrom::contains(self, element)
    }
}

impl<T> Contains<T> for RangeTo<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeTo::contains(self, element)
    }
}

impl<T> Contains<T> for &RangeTo<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeTo::contains(self, element)
    }
}

impl<T> Contains<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeInclusive::contains(self, element)
    }
}

impl<T> Contains<T> for &RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeInclusive::contains(self, element)
    }
}

impl<T> Contains<T> for RangeToInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeToInclusive::contains(self, element)
    }
}

impl<T> Contains<T> for &RangeToInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        RangeToInclusive::contains(self, element)
    }
}

/// The matcher for [`contain_element`] and [`contain_elements`].
///
/// [`contain_element`]: crate::contain_element
/// [`contain_elements`]: crate::contain_elements
#[derive(Debug)]
pub struct ContainElementsMatcher<T> {
    expected: Vec<T>,
}

impl<T> ContainElementsMatcher<T> {
    /// Create a new [`ContainElementsMatcher`] from the expected elements.
    pub fn new(elements: impl Into<Vec<T>>) -> Self {
        Self {
            expected: elements.into(),
        }
    }
}

impl<T, Actual> SimpleMatch<Actual> for ContainElementsMatcher<T>
where
    Actual: Contains<T>,
{
    type Fail = Mismatch<Vec<T>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(self
            .expected
            .iter()
            .all(|expected| actual.contains(expected)))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}

/// The matcher for [`consist_of`].
///
/// [`consist_of`]: crate::consist_of
#[derive(Debug)]
pub struct ConsistOfMatcher<T> {
    expected: Vec<T>,
}

impl<T> ConsistOfMatcher<T> {
    /// Create a new [`ConsistOfMatcher`] from the expected elements.
    pub fn new(elements: impl Into<Vec<T>>) -> Self {
        Self {
            expected: elements.into(),
        }
    }
}

impl<T, Actual> SimpleMatch<Actual> for ConsistOfMatcher<T>
where
    Actual: Contains<T> + Len,
{
    type Fail = Mismatch<Vec<T>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.len() == self.expected.len()
            && self
                .expected
                .iter()
                .all(|expected| actual.contains(expected)))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}

/// The matcher for [`be_in`].
///
/// [`be_in`]: crate::be_in
#[derive(Debug)]
pub struct BeInMatcher<Collection> {
    collection: Collection,
}

impl<Collection> BeInMatcher<Collection> {
    /// Create a new [`BeInMatcher`] from the expected collection.
    pub fn new(collection: Collection) -> Self {
        Self { collection }
    }
}

impl<Collection, Actual> SimpleMatch<Actual> for BeInMatcher<Collection>
where
    Collection: Contains<Actual>,
{
    type Fail = Mismatch<Collection, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(self.collection.contains(actual))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.collection,
            actual,
        }
    }
}
