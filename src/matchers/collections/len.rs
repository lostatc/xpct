use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{OsStr, OsString};

use crate::core::Match;

use crate::matchers::{Expectation, Mismatch};

/// A collection that has a length.
pub trait Len {
    /// The length of the collection.
    fn len(&self) -> usize;

    /// Whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, const N: usize> Len for [T; N] {
    fn len(&self) -> usize {
        N
    }

    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T, const N: usize> Len for &[T; N] {
    fn len(&self) -> usize {
        N
    }

    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T> Len for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T> Len for &Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T> Len for VecDeque<T> {
    fn len(&self) -> usize {
        VecDeque::len(self)
    }

    fn is_empty(&self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<T> Len for &VecDeque<T> {
    fn len(&self) -> usize {
        VecDeque::len(self)
    }

    fn is_empty(&self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<T> Len for LinkedList<T> {
    fn len(&self) -> usize {
        LinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        LinkedList::is_empty(self)
    }
}

impl<T> Len for &LinkedList<T> {
    fn len(&self) -> usize {
        LinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        LinkedList::is_empty(self)
    }
}

impl<K, V> Len for HashMap<K, V> {
    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<K, V> Len for &HashMap<K, V> {
    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<T> Len for HashSet<T> {
    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<T> Len for &HashSet<T> {
    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<K, V> Len for BTreeMap<K, V> {
    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }
}

impl<K, V> Len for &BTreeMap<K, V> {
    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }
}

impl<T> Len for BTreeSet<T> {
    fn len(&self) -> usize {
        BTreeSet::len(self)
    }

    fn is_empty(&self) -> bool {
        BTreeSet::is_empty(self)
    }
}

impl<T> Len for &BTreeSet<T> {
    fn len(&self) -> usize {
        BTreeSet::len(self)
    }

    fn is_empty(&self) -> bool {
        BTreeSet::is_empty(self)
    }
}

impl<T> Len for BinaryHeap<T> {
    fn len(&self) -> usize {
        BinaryHeap::len(self)
    }

    fn is_empty(&self) -> bool {
        BinaryHeap::is_empty(self)
    }
}

impl<T> Len for &BinaryHeap<T> {
    fn len(&self) -> usize {
        BinaryHeap::len(self)
    }

    fn is_empty(&self) -> bool {
        BinaryHeap::is_empty(self)
    }
}

impl Len for String {
    fn len(&self) -> usize {
        String::len(self)
    }

    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl Len for &String {
    fn len(&self) -> usize {
        String::len(self)
    }

    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<'a> Len for &'a str {
    fn len(&self) -> usize {
        <str>::len(self)
    }

    fn is_empty(&self) -> bool {
        <str>::is_empty(self)
    }
}

impl<'a> Len for Cow<'a, str> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl<'a> Len for &Cow<'a, str> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl Len for OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }

    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl Len for &OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }

    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl<'a> Len for &'a OsStr {
    fn len(&self) -> usize {
        <OsStr>::len(self)
    }

    fn is_empty(&self) -> bool {
        <OsStr>::is_empty(self)
    }
}

/// The matcher for [`have_len`].
///
/// [`have_len`]: crate::have_len
#[derive(Debug)]
pub struct HaveLenMatcher {
    len: usize,
}

impl HaveLenMatcher {
    /// Create a new [`HaveLenMatcher`] with the expected length.
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl<Actual> Match<Actual> for HaveLenMatcher
where
    Actual: Len,
{
    type Fail = Mismatch<usize, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.len() == self.len)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.len,
            actual,
        }
    }
}

/// The matcher for [`be_empty`].
///
/// [`be_empty`]: crate::be_empty
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct BeEmptyMatcher;

impl BeEmptyMatcher {
    /// Create a new [`BeEmptyMatcher`].
    pub fn new() -> Self {
        Self
    }
}

impl<Actual> Match<Actual> for BeEmptyMatcher
where
    Actual: Len,
{
    type Fail = Expectation<Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.len() == 0)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Expectation { actual }
    }
}
