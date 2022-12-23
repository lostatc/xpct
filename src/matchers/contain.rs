use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use crate::core::SimpleMatch;

use super::Mismatch;

/// A collection that supports testing for membership.
pub trait Contains<T: ?Sized> {
    /// The collection contains the given element.
    fn contains(&self, element: &T) -> bool;
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

impl<T> Contains<T> for LinkedList<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> Contains<T> for VecDeque<T>
where
    T: PartialEq<T> + Eq,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
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

impl<T> Contains<T> for HashSet<T>
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

impl<K, V> Contains<K> for HashMap<K, V>
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

impl<T> Contains<T> for Range<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> Contains<T> for RangeFrom<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> Contains<T> for RangeTo<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> Contains<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> Contains<T> for RangeToInclusive<T>
where
    T: PartialOrd,
{
    fn contains(&self, element: &T) -> bool {
        self.contains(element)
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
