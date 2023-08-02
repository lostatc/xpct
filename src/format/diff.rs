#![cfg(feature = "diff")]

use std::fmt;
use std::marker::PhantomData;

use crate::core::{
    strings, style, Color, Format, Formatter, MatchFailure, Matcher, OutputStyle, TextColor,
    TextStyle,
};
use crate::matchers::{Diff, DiffTag, Diffable, EqDiffMatcher, SLICE_DIFF_KIND, STRING_DIFF_KIND};

const FORMAT_PLACEHOLDER: &str = "%s";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegmentStyle<T> {
    pub insert: T,
    pub delete: T,
    pub equal: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct StringDiffStyle {
    pub style: DiffSegmentStyle<OutputStyle>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SliceDiffStyle {
    pub element_style: DiffSegmentStyle<OutputStyle>,
    pub gutter_char: DiffSegmentStyle<char>,
    pub gutter_style: DiffSegmentStyle<OutputStyle>,
}

impl SliceDiffStyle {
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

impl Default for SliceDiffStyle {
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DiffStyle {
    pub string: StringDiffStyle,
    pub slice: SliceDiffStyle,
}

impl DiffStyle {
    /// The provided styling, used by [`eq_diff`].
    ///
    /// You can use this as a starting point for customizing the provided styling. The value
    /// returned by this method may change and is not part of the public API.
    pub fn provided() -> Self {
        Self {
            string: StringDiffStyle::provided(),
            slice: SliceDiffStyle::provided(),
        }
    }
}

#[derive(Debug)]
pub struct DiffFormat<Actual, Expected> {
    style: DiffStyle,
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> DiffFormat<Actual, Expected> {
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
    type Value = MatchFailure<Diff<Expected::Segment>>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        let diff = match value {
            MatchFailure::Pos(diff) => {
                f.set_style(style::important());
                f.write_str("Expected these to be equal:\n");
                diff
            }
            MatchFailure::Neg(diff) => {
                f.set_style(style::important());
                f.write_str("Expected these to not be equal:\n");
                diff
            }
        };

        f.reset_style();

        match Expected::KIND {
            STRING_DIFF_KIND => {
                f.indented(style::INDENT_LEN, |f| {
                    for segment in diff {
                        let (format, style) = match segment.tag() {
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

                        let segment_string = Expected::repr(segment.value());
                        let formatted_segment =
                            format.replacen(FORMAT_PLACEHOLDER, &segment_string, 1);

                        f.set_style(style);
                        f.write_str(&formatted_segment);
                    }

                    Ok(())
                })?;

                Ok(())
            }
            SLICE_DIFF_KIND => {
                f.indented(style::INDENT_LEN, |f| {
                    f.write_char('[');
                    f.write_char('\n');

                    for segment in diff {
                        let (gutter, gutter_style, element_style) = match segment.tag() {
                            DiffTag::Insert => (
                                self.style.slice.gutter_char.insert,
                                self.style.slice.gutter_style.insert.clone(),
                                self.style.slice.element_style.insert.clone(),
                            ),
                            DiffTag::Delete => (
                                self.style.slice.gutter_char.delete,
                                self.style.slice.gutter_style.delete.clone(),
                                self.style.slice.element_style.delete.clone(),
                            ),
                            DiffTag::Equal => (
                                self.style.slice.gutter_char.equal,
                                self.style.slice.gutter_style.equal.clone(),
                                self.style.slice.element_style.equal.clone(),
                            ),
                        };

                        f.set_style(gutter_style);
                        f.write_char(gutter);
                        f.reset_style();

                        // Leave room for the gutter char.
                        f.write_str(strings::whitespace(style::indent_len(1) as usize - 1));

                        f.set_style(element_style);
                        f.write_str(Expected::repr(segment.value()));
                        f.reset_style();

                        f.write_char(',');
                        f.write_char('\n');
                    }

                    f.write_char(']');

                    Ok(())
                })?;

                Ok(())
            }
            _ => Err(crate::Error::msg(format!(
                "this is not a supported diffable kind: {}",
                Expected::KIND,
            ))),
        }
    }
}

pub fn eq_diff<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: Diffable<Actual> + fmt::Debug + 'a,
{
    Matcher::new(
        EqDiffMatcher::new(expected),
        DiffFormat::<Actual, Expected>::new(DiffStyle::provided()),
    )
}
