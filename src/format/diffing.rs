use std::fmt;
use std::marker::PhantomData;

use crate::core::{
    strings, style, Color, Format, Formatter, MatchFailure, Matcher, OutputStyle, TextColor,
    TextStyle,
};
use crate::matchers::diff::{Diff, DiffKind, DiffTag, Diffable, EqDiffMatcher};

const FORMAT_PLACEHOLDER: &str = "%s";

/// A configuration option for [`DiffStyle`].
///
/// This represents a generic value that differs when an element is added, removed, or remains
/// unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegmentStyle<T> {
    /// The element is in the actual value but not the expected value.
    pub insert: T,

    /// The element is in the expected value but not the actual value.
    pub delete: T,

    /// The element is the same in both the actual and expected values.
    pub equal: T,
}

/// The styling for string diffs when using [`DiffFormat`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct StringDiffStyle {
    /// The text styling to use for substrings that are added or removed.
    pub style: DiffSegmentStyle<OutputStyle>,

    /// Format strings used to wrap additions and deletions.
    ///
    /// Out of the box, string diffs are impossible to read with text styling disabled. Since they
    /// diff by character/grapheme instead of by line, there's nowhere to put a gutter like `git
    /// diff` has. You can add strings before and after additions and deletions to make it possible
    /// to see the changes without text styling.
    ///
    /// Each format string replaces the first occurrence of `%s` with the added or removed text.
    /// Note that if the format string doesn't contain a `%s`, that text will not be shown.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::format::diff::DiffSegmentStyle;
    ///
    /// let style = DiffSegmentStyle {
    ///     insert: String::from("+(%s)"),
    ///     delete: String::from("-(%s)"),
    ///     equal: String::from("%s"),
    /// };
    /// ```
    pub format: DiffSegmentStyle<String>,
}

impl StringDiffStyle {
    /// The provided styling, used by [`eq_diff`].
    ///
    /// You can use this as a starting point for customizing the provided styling. The value
    /// returned by this method may change and is not part of the public API.
    pub fn provided() -> Self {
        Self {
            style: DiffSegmentStyle {
                insert: OutputStyle {
                    style: TextStyle::BOLD | TextStyle::REVERSED,
                    color: TextColor {
                        fg: Some(Color::BrightGreen),
                        bg: None,
                    },
                },
                delete: OutputStyle {
                    style: TextStyle::BOLD | TextStyle::UNDERLINE,
                    color: TextColor {
                        fg: Some(Color::BrightRed),
                        bg: None,
                    },
                },
                equal: OutputStyle::default(),
            },
            format: DiffSegmentStyle {
                insert: String::from("%s"),
                delete: String::from("%s"),
                equal: String::from("%s"),
            },
        }
    }
}

impl Default for StringDiffStyle {
    fn default() -> Self {
        Self {
            style: DiffSegmentStyle {
                insert: OutputStyle::default(),
                delete: OutputStyle::default(),
                equal: OutputStyle::default(),
            },
            format: DiffSegmentStyle {
                insert: String::from("%s"),
                delete: String::from("%s"),
                equal: String::from("%s"),
            },
        }
    }
}

/// The styling for diffs of collections when using [`DiffFormat`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CollectionDiffStyle {
    /// The text styling to use for each element in the collection.
    ///
    /// You can change the styling of elements to represent values that have been added or removed.
    pub element_style: DiffSegmentStyle<OutputStyle>,

    /// The characters to show in the gutter to represent changes.
    ///
    /// The diff output shows '+' and '-' characters in the gutter, like `git diff`. You can
    /// customize which characters are used.
    pub gutter_char: DiffSegmentStyle<char>,

    /// The text styling to use for the characters in the gutter.
    pub gutter_style: DiffSegmentStyle<OutputStyle>,
}

impl CollectionDiffStyle {
    /// The provided styling, used by [`eq_diff`].
    ///
    /// You can use this as a starting point for customizing the provided styling. The value
    /// returned by this method may change and is not part of the public API.
    pub fn provided() -> Self {
        Self {
            element_style: DiffSegmentStyle {
                insert: OutputStyle {
                    style: TextStyle::BOLD,
                    color: TextColor {
                        fg: Some(Color::BrightGreen),
                        bg: None,
                    },
                },
                delete: OutputStyle {
                    style: TextStyle::BOLD,
                    color: TextColor {
                        fg: Some(Color::BrightRed),
                        bg: None,
                    },
                },
                equal: OutputStyle::default(),
            },
            gutter_char: DiffSegmentStyle {
                insert: '+',
                delete: '-',
                equal: ' ',
            },
            gutter_style: DiffSegmentStyle {
                insert: OutputStyle {
                    style: TextStyle::empty(),
                    color: TextColor {
                        fg: Some(Color::BrightGreen),
                        bg: None,
                    },
                },
                delete: OutputStyle {
                    style: TextStyle::empty(),
                    color: TextColor {
                        fg: Some(Color::BrightRed),
                        bg: None,
                    },
                },
                equal: OutputStyle::default(),
            },
        }
    }
}

impl Default for CollectionDiffStyle {
    fn default() -> Self {
        Self {
            element_style: DiffSegmentStyle {
                insert: OutputStyle::default(),
                delete: OutputStyle::default(),
                equal: OutputStyle::default(),
            },
            gutter_char: DiffSegmentStyle {
                insert: ' ',
                delete: ' ',
                equal: ' ',
            },
            gutter_style: DiffSegmentStyle {
                insert: OutputStyle::default(),
                delete: OutputStyle::default(),
                equal: OutputStyle::default(),
            },
        }
    }
}

/// The style sheet for [`DiffFormat`].
///
/// If the provided text styling for diffs is inaccessible for you, or you prefer to have text
/// styling disabled, you can use this style sheet to customize the styling of diffs.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DiffStyle {
    /// The styling for string diffs.
    pub string: StringDiffStyle,

    /// The styling for diffs of collections.
    pub collection: CollectionDiffStyle,
}

impl DiffStyle {
    /// The provided styling, used by [`eq_diff`].
    ///
    /// You can use this as a starting point for customizing the provided styling. The value
    /// returned by this method may change and is not part of the public API.
    pub fn provided() -> Self {
        Self {
            string: StringDiffStyle::provided(),
            collection: CollectionDiffStyle::provided(),
        }
    }
}

/// A formatter for [`Diff`] values.
///
/// [`Diff`]: crate::matchers::diff::Diff
#[derive(Debug)]
pub struct DiffFormat<Actual, Expected> {
    style: DiffStyle,
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> DiffFormat<Actual, Expected> {
    /// Create a new [`DiffFormat`] from the given style sheet.
    pub fn new(style: DiffStyle) -> Self {
        Self {
            style,
            marker: PhantomData,
        }
    }
}

impl<Actual, Expected> Format for DiffFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: Diffable<Actual> + fmt::Debug,
{
    type Value = MatchFailure<Diff>;

    fn fmt(&self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        let diff = value.unwrap();

        f.set_style(style::important());

        if value.is_pos() {
            f.write_str("Expected these to be equal:\n");
        } else {
            f.write_str("Expected these to not be equal:\n");
        }

        f.reset_style();

        match Expected::KIND {
            DiffKind::String => {
                f.indented(style::indent(1), |f| {
                    for segment in diff {
                        let (format, style) = match segment.tag {
                            DiffTag::Insert => (
                                self.style.string.format.insert.clone(),
                                self.style.string.style.insert.clone(),
                            ),
                            DiffTag::Delete => (
                                self.style.string.format.delete.clone(),
                                self.style.string.style.delete.clone(),
                            ),
                            DiffTag::Equal => (
                                self.style.string.format.equal.clone(),
                                self.style.string.style.equal.clone(),
                            ),
                        };

                        let formatted_segment =
                            format.replacen(FORMAT_PLACEHOLDER, &segment.value, 1);

                        f.set_style(style);
                        f.write_str(&formatted_segment);
                    }

                    Ok(())
                })?;

                Ok(())
            }
            DiffKind::Slice | DiffKind::Set | DiffKind::Map => {
                f.indented(style::indent(1), |f| {
                    match Expected::KIND {
                        DiffKind::Slice => f.write_char('['),
                        DiffKind::Set | DiffKind::Map => f.write_char('{'),
                        _ => unreachable!(),
                    };

                    f.write_char('\n');

                    for segment in diff {
                        let (gutter, gutter_style, element_style) = match segment.tag {
                            DiffTag::Insert => (
                                self.style.collection.gutter_char.insert,
                                self.style.collection.gutter_style.insert.clone(),
                                self.style.collection.element_style.insert.clone(),
                            ),
                            DiffTag::Delete => (
                                self.style.collection.gutter_char.delete,
                                self.style.collection.gutter_style.delete.clone(),
                                self.style.collection.element_style.delete.clone(),
                            ),
                            DiffTag::Equal => (
                                self.style.collection.gutter_char.equal,
                                self.style.collection.gutter_style.equal.clone(),
                                self.style.collection.element_style.equal.clone(),
                            ),
                        };

                        f.set_style(gutter_style);
                        f.write_char(' ');
                        f.write_char(gutter);
                        f.reset_style();

                        // Leave room for the gutter char.
                        f.write_str(strings::whitespace(style::indent_len(1) as usize - 2));

                        f.indented_hanging(style::indent(1), |f| {
                            f.set_style(element_style);
                            f.write_str(&segment.value);
                            f.reset_style();

                            Ok(())
                        })?;

                        f.write_char(',');
                        f.write_char('\n');
                    }

                    match Expected::KIND {
                        DiffKind::Slice => f.write_char(']'),
                        DiffKind::Set | DiffKind::Map => f.write_char('}'),
                        _ => unreachable!(),
                    };

                    Ok(())
                })?;

                Ok(())
            }
            DiffKind::Custom(name) => Err(crate::Error::msg(format!(
                "this is not a supported diffable kind: {name}",
            ))),
        }
    }
}

/// Succeeds when the actual value equals the expected value and shows a diff otherwise.
///
/// This matcher is functionally identical to [`equal`], except it shows a diff if the values are
/// not equal. You can use this matcher with any type that implements [`Diffable`], and you can
/// implement [`Diffable`] for your own types.
///
/// Out of the box, you can diff strings, slices, sets, and maps.
///
/// # Examples
///
/// ```
/// use xpct::{expect, eq_diff};
///
/// expect!("diffing strings").to(eq_diff("diffing strings"));
/// ```
///
/// ```
/// use xpct::{expect, eq_diff};
///
/// expect!(["slices", "too"]).to(eq_diff(["slices", "too"]));
/// ```
///
/// # Custom Styling
///
/// This matcher relies on text styling to distinguish added and removed values in string diffs. If
/// the provided text styling for diffs is inaccessible for you, or you prefer to have text styling
/// disabled, you can override the provided styling.
///
/// To do this, write a custom style sheet using [`DiffStyle`] and write your own matcher function
/// that calls [`EqDiffMatcher`].
///
/// This example makes string diffs readable without text styling:
///
/// ```
/// use std::fmt;
///
/// use xpct::core::Matcher;
/// use xpct::format::diff::{DiffFormat, DiffSegmentStyle, DiffStyle};
/// use xpct::matchers::diff::{Diffable, EqDiffMatcher};
///
/// pub fn eq_diff<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
/// where
///     Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
///     Expected: Diffable<Actual> + fmt::Debug + 'a,
/// {
///     let mut custom_style = DiffStyle::provided();
///
///     custom_style.string.format = DiffSegmentStyle {
///         insert: String::from("+(%s)"),
///         delete: String::from("-(%s)"),
///         equal: String::from("%s"),
///     };
///
///     Matcher::new(
///         EqDiffMatcher::new(expected),
///         DiffFormat::<Actual, Expected>::new(custom_style),
///     )
/// }
/// ```
///
/// [`equal`]: crate::equal
/// [`Diffable`]: crate::matchers::diff::Diffable
/// [`EqDiffMatcher`]: crate::matchers::diff::EqDiffMatcher
pub fn eq_diff<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + Diffable<Actual> + 'a,
{
    Matcher::new(
        EqDiffMatcher::new(expected),
        DiffFormat::<Actual, Expected>::new(DiffStyle::provided()),
    )
}

#[cfg(test)]
mod tests {
    use super::eq_diff;
    use crate::expect;

    #[test]
    fn succeeds_when_equal() {
        expect!("some string").to(eq_diff("some string"));
        expect!(["some", "slice"]).to(eq_diff(["some", "slice"]));
    }

    #[test]
    fn succeeds_when_not_equal() {
        expect!("some string").to_not(eq_diff("a different string"));
        expect!(["some", "slice"]).to_not(eq_diff(["different", "slice"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_equal() {
        expect!("some string").to_not(eq_diff("some string"));
        expect!(["some", "slice"]).to_not(eq_diff(["some", "slice"]));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_equal() {
        expect!("some string").to(eq_diff("a different string"));
        expect!(["some", "slice"]).to(eq_diff(["different", "slice"]));
    }
}
