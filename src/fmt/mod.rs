use std::fmt;

use crate::matchers::Mismatch;
use crate::Formatter;

pub fn indexed_list<'a, Iter>(f: &mut Formatter, list: Iter)
where
    Iter: IntoIterator<Item = &'a str>,
    Iter::IntoIter: ExactSizeIterator,
{
    let iter = list.into_iter();
    let digits = iter.len().to_string().len();
    let new_indent = f.indent() + digits as u32 + 4;

    for (i, item) in iter.enumerate() {
        f.set_indent(0);
        f.write_str(format!("[{:0digits$}]  ", i, digits = digits));
        f.set_indent(new_indent);
        f.write_str(item);
        f.writeln();
        f.writeln();
    }
}

#[derive(Debug)]
pub struct Labeled<'a> {
    pub label: &'a str,
    pub element: &'a str,
}

pub fn labeled_list<'a, Iter>(f: &mut Formatter, list: Iter)
where
    Iter: IntoIterator<Item = Labeled<'a>>,
{
    for item in list {
        f.set_indent(0);
        f.write_str(item.label);
        f.writeln();
        f.set_indent(2);
        f.write_str(item.element);
        f.writeln();
    }
}

#[derive(Debug)]
pub struct LabeledMismatch<'a, Actual, Expected> {
    pub actual_label: &'a str,
    pub expected_label: &'a str,
    pub mismatch: &'a Mismatch<Actual, Expected>,
}

pub fn labeled_mismatch<'a, Actual, Expected>(
    f: &mut Formatter,
    mismatch: LabeledMismatch<Actual, Expected>,
) where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    labeled_list(
        f,
        [
            Labeled {
                label: mismatch.expected_label,
                element: format!("{:?}", mismatch.mismatch.expected).trim(),
            },
            Labeled {
                label: mismatch.actual_label,
                element: format!("{:?}", mismatch.mismatch.actual).trim(),
            },
        ],
    )
}
