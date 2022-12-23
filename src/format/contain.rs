use std::fmt;

use crate::core::Matcher;
use crate::matchers::{ContainElementsMatcher, Contains};

use super::MismatchFormat;

/// Succeeds when the actual value contains the given element.
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

#[cfg(test)]
mod tests {
    use super::contain_element;
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
}
