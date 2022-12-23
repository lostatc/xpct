use std::fmt;

use crate::core::Matcher;
use crate::matchers::{ConsistOfMatcher, ContainElementsMatcher, Contains, Len};

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
/// expect!(vec!["foo", "bar"]).to(contain_element("foo"));
/// ```
pub fn contain_element<'a, T, Actual>(element: T) -> Matcher<'a, Actual, Actual>
where
    T: fmt::Debug + 'a,
    Actual: fmt::Debug + Contains<T> + 'a,
{
    Matcher::simple(
        ContainElementsMatcher::new(vec![element]),
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
/// expect!("foo").to(contain_elements(vec!['f', 'o']));
/// expect!(vec!["foo", "bar"]).to(contain_elements(vec!["foo", "bar"]));
/// ```
pub fn contain_elements<'a, T, Actual>(elements: impl Into<Vec<T>>) -> Matcher<'a, Actual, Actual>
where
    T: fmt::Debug + 'a,
    Actual: fmt::Debug + Contains<T> + 'a,
{
    Matcher::simple(
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
/// use xpct::{expect, contain_elements};
///
/// expect!("foo").to(contain_elements(vec!['o', 'f', 'o']));
/// expect!(vec!["foo", "bar", "baz"]).to(contain_elements(vec!["bar", "foo", "baz"]));
/// ```
pub fn consist_of<'a, T, Actual>(elements: impl Into<Vec<T>>) -> Matcher<'a, Actual, Actual>
where
    T: fmt::Debug + 'a,
    Actual: fmt::Debug + Contains<T> + Len + 'a,
{
    Matcher::simple(
        ConsistOfMatcher::new(elements),
        MismatchFormat::new("to consist of elements", "to consist of elements"),
    )
}

#[cfg(test)]
mod tests {
    use super::{consist_of, contain_element};
    use crate::expect;

    #[test]
    fn succeeds_when_contains_elements() {
        expect!(vec!["foo"]).to(contain_element("foo"));
    }

    #[test]
    fn succeeds_when_not_contains_elements() {
        expect!(vec!["foo"]).to_not(contain_element("not contained in the collection"));
    }

    #[test]
    #[should_panic]
    fn fails_when_contains_elements() {
        expect!(vec!["foo"]).to_not(contain_element("foo"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_contains_elements() {
        expect!(vec!["foo"]).to(contain_element("not contained in the collection"));
    }

    #[test]
    fn succeeds_when_consists_of() {
        expect!(vec!["foo", "bar"]).to(consist_of(vec!["bar", "foo"]));
    }

    #[test]
    fn succeeds_when_not_consists_of() {
        expect!(vec!["foo", "bar"]).to_not(consist_of(vec!["foo"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_consists_of() {
        expect!(vec!["foo", "bar"]).to_not(consist_of(vec!["bar", "foo"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_consists_of() {
        expect!(vec!["foo", "bar"]).to(consist_of(vec!["foo"]));
    }
}
