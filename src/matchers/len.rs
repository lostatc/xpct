use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ffi::{OsStr, OsString},
};

use crate::core::SimpleMatch;

use super::Mismatch;

/// A collection that has a length.
pub trait HasLen {
    /// The length of the collection.
    fn len(&self) -> usize;

    /// Whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> HasLen for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T, const N: usize> HasLen for [T; N] {
    fn len(&self) -> usize {
        N
    }

    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T> HasLen for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasLen for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasLen for LinkedList<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> HasLen for HashMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasLen for HashSet<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> HasLen for BTreeMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasLen for BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasLen for BinaryHeap<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl HasLen for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<'a> HasLen for &'a str {
    fn len(&self) -> usize {
        <str>::len(self)
    }

    fn is_empty(&self) -> bool {
        <str>::is_empty(self)
    }
}

impl<'a> HasLen for Cow<'a, str> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl HasLen for OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }

    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl<'a> HasLen for &'a OsStr {
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

impl<Actual> SimpleMatch<Actual> for HaveLenMatcher
where
    Actual: HasLen,
{
    type Fail = Mismatch<usize, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.len() == self.len)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: actual.len(),
            actual,
        }
    }
}
