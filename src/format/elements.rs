use std::fmt;

use crate::core::{style, Matcher};
use crate::matchers::MatchElementsMatcher;

use super::{HeaderFormat, SomeFailuresFormat};

/// Tests each element of the actual value against a different matcher.
///
/// This accepts an iterator of matchers. The first matcher tests the first element of the actual
/// value, the second matcher tests the second element of the actual value, and so on.
///
/// This matcher does not fail just because the actual iterator and the iterator of matchers have
/// different lengths. If the actual iterator is shorter, it ignores the excess matchers. If the
/// iterator of matchers is shorter, it ignores the excess elements.
///
/// # Examples
///
/// ```
/// use xpct::{be_in, equal, expect, have_prefix, match_elements};
///
/// let items = vec!["apple", "banana", "cucumber"];
///
/// expect!(items).to(match_elements([
///     equal("apple"),
///     be_in(["banana", "orange"]),
///     have_prefix("c"),
/// ]));
/// ```
///
/// If you want to test that the two iterators have the same length, you should use [`have_len`].
///
/// ```
/// use xpct::{be_in, equal, expect, have_len, have_prefix, match_elements};
///
/// let items = vec!["apple", "banana", "cucumber"];
///
/// expect!(items)
///     .to(have_len(3))
///     .to(match_elements([
///         equal("apple"),
///         be_in(["banana", "orange"]),
///         have_prefix("c"),
///     ]));
/// ```
///
/// [`have_len`]: crate::have_len
pub fn match_elements<'a, PosOut, NegOut, IntoIter>(
    matchers: impl IntoIterator<Item = Matcher<'a, IntoIter::Item, PosOut, NegOut>> + 'a,
) -> Matcher<'a, IntoIter, Vec<PosOut>, Vec<NegOut>>
where
    IntoIter: fmt::Debug + IntoIterator + 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::transform(
        MatchElementsMatcher::new(matchers),
        HeaderFormat::new(
            SomeFailuresFormat::new(),
            style::ALL_OK_HEADER,
            style::AT_LESAT_ONE_OK_HEADER,
        ),
    )
}
