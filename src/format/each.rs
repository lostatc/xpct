use crate::core::{strings, style, Format, Formatter, PosMatcher};
use crate::matchers::{EachContext, EachMatcher, SomeFailures};

use super::HeaderFormat;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SomeFailuresFormat;

impl SomeFailuresFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        let num_failures = value.len();
        let failure_indent = strings::int_len(num_failures, 10) + 4;

        for (i, maybe_fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}[{}]  ",
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            match maybe_fail {
                Some(fail) => {
                    f.set_style(style::failure());
                    f.write_str(style::FAILED_MSG);
                    f.reset_style();
                    f.write_char('\n');

                    f.write_fmt(fail.into_fmt().indented(failure_indent));
                }
                None => {
                    f.set_style(style::success());
                    f.write_str(style::MATCHED_MSG);
                    f.reset_style();
                    f.write_char('\n');
                }
            }

            f.write_char('\n');
        }

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn each<'a, T>(block: impl FnOnce(&mut EachContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(
        EachMatcher::new(block),
        HeaderFormat::new(SomeFailuresFormat::new(), "Expected all of these to match:"),
    )
}
