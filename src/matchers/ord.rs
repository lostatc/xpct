use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;

use crate::core::Match;

use super::{Expectation, Mismatch};

/// Which inequality test to perform with [`OrdMatcher`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Inequality {
    /// [`PartialOrd::lt`]
    Less,

    /// [`PartialOrd::le`]
    LessOrEqual,

    /// [`PartialOrd::gt`]
    Greater,

    /// [`PartialOrd::ge`]
    GreaterOrEqual,
}

/// The matcher for [`be_lt`], [`be_le`], [`be_gt`], and [`be_ge`].
///
/// [`be_lt`]: crate::be_lt
/// [`be_le`]: crate::be_le
/// [`be_gt`]: crate::be_gt
/// [`be_ge`]: crate::be_ge
#[derive(Debug)]
pub struct OrdMatcher<Expected> {
    expected: Expected,
    kind: Inequality,
}

impl<Expected> OrdMatcher<Expected> {
    /// Create a new [`OrdMatcher`].
    ///
    /// This accepts a `kind` which determines whether the behavior is `<`, `<=`, `>`, or `>=`.
    pub fn new(expected: Expected, kind: Inequality) -> Self {
        Self { expected, kind }
    }
}

impl<Expected, Actual> Match<Actual> for OrdMatcher<Expected>
where
    Actual: PartialOrd<Expected>,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(match self.kind {
            Inequality::Less => actual < &self.expected,
            Inequality::LessOrEqual => actual <= &self.expected,
            Inequality::Greater => actual > &self.expected,
            Inequality::GreaterOrEqual => actual >= &self.expected,
        })
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}

/// A sort order, either ascending or descending.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SortOrder {
    #[allow(missing_docs)]
    Asc,

    #[allow(missing_docs)]
    Desc,
}

/// The matcher for [`be_sorted_asc`] and [`be_sorted_desc`].
///
/// [`be_sorted_asc`]: crate::be_sorted_asc
/// [`be_sorted_desc`]: crate::be_sorted_desc
#[derive(Debug)]
#[non_exhaustive]
pub struct BeSortedMatcher<T> {
    order: SortOrder,
    marker: PhantomData<T>,
}

impl<T> BeSortedMatcher<T> {
    /// Create a new [`BeSortedMatcher`] from the given sort order.
    pub fn new(order: SortOrder) -> Self {
        Self {
            order,
            marker: PhantomData,
        }
    }
}

impl<T, Actual> Match<Actual> for BeSortedMatcher<T>
where
    T: Ord,
    Actual: AsRef<[T]>,
{
    type Fail = Expectation<Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().windows(2).all(|window| match self.order {
            SortOrder::Asc => window[0] <= window[1],
            SortOrder::Desc => window[0] >= window[1],
        }))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Expectation { actual }
    }
}

type BoxSortPredicate<'a, T> = Box<dyn Fn(&T, &T) -> Ordering + 'a>;

/// The matcher for [`be_sorted_by`].
///
/// [`be_sorted_by`]: crate::be_sorted_by
#[non_exhaustive]
pub struct BeSortedByMatcher<'a, T> {
    predicate: BoxSortPredicate<'a, T>,
    marker: PhantomData<T>,
}

impl<'a, T> fmt::Debug for BeSortedByMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BeSortedByMatcher").finish_non_exhaustive()
    }
}

impl<'a, T> BeSortedByMatcher<'a, T> {
    /// Create a new [`BeSortedByMatcher`] from the given sort predicate.
    pub fn new(predicate: impl Fn(&T, &T) -> Ordering + 'a) -> Self {
        Self {
            predicate: Box::new(predicate),
            marker: PhantomData,
        }
    }
}

impl<'a, T, Actual> Match<Actual> for BeSortedByMatcher<'a, T>
where
    T: Ord,
    Actual: AsRef<[T]>,
{
    type Fail = Expectation<Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().windows(2).all(|window| {
            let ordering = (self.predicate)(&window[0], &window[1]);
            ordering == Ordering::Less || ordering == Ordering::Equal
        }))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Expectation { actual }
    }
}
