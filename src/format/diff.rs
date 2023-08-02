#![cfg(feature = "diff")]

use std::fmt;
use std::marker::PhantomData;

use crate::core::{
    style, Color, Format, Formatter, MatchFailure, Matcher, OutputStyle, TextColor, TextStyle,
};
use crate::matchers::{Diff, DiffTag, Diffable, EqDiffMatcher, SLICE_DIFF_KIND, STRING_DIFF_KIND};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSegmentStyle<T> {
    pub insert: T,
    pub delete: T,
    pub equal: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct StringDiffStyle {
    pub styles: DiffSegmentStyle<OutputStyle>,
    pub format: DiffSegmentStyle<String>,
}

impl Default for StringDiffStyle {
    fn default() -> Self {
        Self {
            styles: DiffSegmentStyle {
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
    pub styles: DiffSegmentStyle<OutputStyle>,
    pub gutter: DiffSegmentStyle<char>,
}

impl Default for SliceDiffStyle {
    fn default() -> Self {
        Self {
            styles: DiffSegmentStyle {
                insert: OutputStyle::default(),
                delete: OutputStyle::default(),
                equal: OutputStyle::default(),
            },
            gutter: DiffSegmentStyle {
                insert: ' ',
                delete: ' ',
                equal: ' ',
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DiffStyle {
    pub string: StringDiffStyle,
    pub slice: SliceDiffStyle,
}

fn default_style() -> DiffStyle {
    let segment_style = DiffSegmentStyle {
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
    };

    DiffStyle {
        string: StringDiffStyle {
            styles: segment_style.clone(),
            format: DiffSegmentStyle {
                insert: String::from("%s"),
                delete: String::from("%s"),
                equal: String::from("%s"),
            },
        },
        slice: SliceDiffStyle {
            styles: segment_style,
            gutter: DiffSegmentStyle {
                insert: '+',
                delete: '-',
                equal: ' ',
            },
        },
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
                f.indented(style::INDENT_LEN, |inner_f| {
                    for segment in diff {
                        let style = match segment.tag() {
                            DiffTag::Insert => self.style.string.styles.insert.clone(),
                            DiffTag::Delete => self.style.string.styles.delete.clone(),
                            DiffTag::Equal => self.style.string.styles.equal.clone(),
                        };

                        inner_f.set_style(style);

                        inner_f.write_str(Expected::repr(segment.value()));
                    }

                    Ok(())
                })?;

                Ok(())
            }
            SLICE_DIFF_KIND => todo!(),
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
        DiffFormat::<Actual, Expected>::new(default_style()),
    )
}
