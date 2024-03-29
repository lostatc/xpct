use std::borrow::Borrow;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::collections::{
    BeInMatcher, ConsistOfMatcher, ContainElementsMatcher, Contains, Len,
};

use super::MismatchFormat;

/// Succeeds when the actual value contains the given element.
///
/// You can use this matcher for your own types by implementing [`Contains`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{expect, contain_element};
///
/// expect!("foo").to(contain_element('f'));
/// expect!(["foo", "bar"]).to(contain_element("foo"));
/// ```
pub fn contain_element<'a, T, Actual>(element: T) -> Matcher<'a, Actual, Actual>
where
    T: fmt::Debug + Clone + 'a,
    Actual: fmt::Debug + Contains<T> + 'a,
{
    Matcher::new(
        ContainElementsMatcher::new([element]),
        MismatchFormat::new("to contain elements", "to not contain elements"),
    )
}

/// Succeeds when the actual value contains all the given elements.
///
/// You can use this matcher for your own types by implementing [`Contains`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{expect, contain_elements};
///
/// expect!("foobar").to(contain_elements(['f', 'b']));
/// expect!(["foo", "bar", "baz"]).to(contain_elements(["bar", "foo"]));
/// ```
pub fn contain_elements<'a, T, Expected, Actual>(elements: Expected) -> Matcher<'a, Actual, Actual>
where
    T: fmt::Debug + 'a,
    Actual: fmt::Debug + Contains<T> + 'a,
    Expected: fmt::Debug + IntoIterator + Clone + 'a,
    Expected::Item: Borrow<T>,
{
    Matcher::new(
        ContainElementsMatcher::new(elements),
        MismatchFormat::new("to contain elements", "to not contain elements"),
    )
}

/// Succeeds when the actual value contains exactly the given elements, in any order.
///
/// You can use this matcher for your own types by implementing [`Contains`] and [`Len`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{expect, consist_of};
///
/// expect!(["foo", "bar", "baz"]).to(consist_of(["bar", "foo", "baz"]));
/// ```
pub fn consist_of<'a, T, Expected, Actual>(elements: Expected) -> Matcher<'a, Actual, Actual>
where
    T: 'a,
    Expected: fmt::Debug + Contains<T> + Len + 'a,
    Actual: fmt::Debug + IntoIterator + Len + Clone + 'a,
    Actual::Item: Borrow<T>,
{
    Matcher::new(
        ConsistOfMatcher::new(elements),
        MismatchFormat::new("to consist of elements", "to consist of elements"),
    )
}

/// Succeeds when the actual value is contained in the expected collection.
///
/// You can use this matcher for your own types by implementing [`Contains`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{be_in, expect};
///
/// expect!("Mañana").to(be_in(["Evrart", "Mañana"]));
/// expect!('C').to(be_in("Cuno"));
/// expect!(50).to(be_in(41..57));
/// ```
pub fn be_in<'a, Collection, Actual>(collection: Collection) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + 'a,
    Collection: fmt::Debug + Contains<Actual> + 'a,
{
    Matcher::new(
        BeInMatcher::new(collection),
        MismatchFormat::new("to be in", "to not be in"),
    )
}

#[cfg(test)]
mod tests {
    use super::{be_in, consist_of, contain_element};
    use crate::expect;

    #[test]
    fn succeeds_when_contains_elements() {
        expect!(["foo"]).to(contain_element("foo"));
    }

    #[test]
    fn succeeds_when_not_contains_elements() {
        expect!(["foo"]).to_not(contain_element("not contained in the collection"));
    }

    #[test]
    #[should_panic]
    fn fails_when_contains_elements() {
        expect!(["foo"]).to_not(contain_element("foo"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_contains_elements() {
        expect!(["foo"]).to(contain_element("not contained in the collection"));
    }

    #[test]
    fn succeeds_when_consists_of() {
        expect!(["foo", "bar"]).to(consist_of(["bar", "foo"]));
    }

    #[test]
    fn succeeds_when_not_consists_of() {
        expect!(["foo", "bar"]).to_not(consist_of(["foo"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_consists_of() {
        expect!(["foo", "bar"]).to_not(consist_of(["bar", "foo"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_consists_of() {
        expect!(["foo", "bar"]).to(consist_of(["foo"]));
    }

    #[test]
    fn succeeds_when_in_collection() {
        expect!("foo").to(be_in(["foo", "bar"]));
    }

    #[test]
    fn succeeds_when_not_in_collection() {
        expect!("not in collection").to_not(be_in(["foo", "bar"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_in_collection() {
        expect!("foo").to_not(be_in(["foo", "bar"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_in_collection() {
        expect!("not in collection").to(be_in(["foo", "bar"]));
    }
}
