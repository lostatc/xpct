use std::borrow::Cow;

const PREFIX_CACHE: &str = "                                                                ";

#[cfg(feature = "color")]
mod with_color {

    use crate::core::OutputStyle;

    /// A styled segment of a string.
    #[derive(Debug, Default, Clone)]
    pub struct OutputSegment {
        pub buf: String,
        pub style: OutputStyle,
    }

    /// Indent each line of the string represented by the given list of styled segments.
    ///
    /// This ensures that the whitespace used for indentation is unstyled.
    pub fn indent_segments(
        segments: Vec<OutputSegment>,
        prefix: &str,
        hanging: bool,
    ) -> Vec<OutputSegment> {
        // We know that we'll probably need more segments than we were given, but we don't know
        // exactly how many yet.
        let mut new_segments = Vec::with_capacity(segments.len() * 2);

        // Write the indentation spaces with no formatting. Even though colors won't appear on
        // whitespace, some text styles will.
        if !hanging {
            new_segments.push(OutputSegment {
                buf: prefix.to_string(),
                style: OutputStyle::default(),
            });
        }

        let non_empty_segments = segments
            .into_iter()
            .filter(|segment| !segment.buf.is_empty())
            .collect::<Vec<_>>();

        for (segment_index, segment) in non_empty_segments.iter().enumerate() {
            if segment.buf.is_empty() {
                continue;
            }

            let is_last_segment = segment_index == non_empty_segments.len() - 1;

            // This works for both '\n' line endings and '\r\n' line endings.
            let segment_ends_in_newline =
                matches!(segment.buf.chars().next_back(), Some(last_char) if last_char == '\n');

            let lines = segment.buf.lines().collect::<Vec<_>>();
            let num_lines = lines.len();

            for (line_index, line) in lines.into_iter().enumerate() {
                let is_last_line_in_segment = line_index == num_lines - 1;
                let needs_newline = !is_last_line_in_segment || segment_ends_in_newline;

                let mut owned_line = line.to_owned();

                if needs_newline {
                    owned_line.push('\n');
                }

                new_segments.push(OutputSegment {
                    buf: owned_line,
                    style: segment.style.clone(),
                });

                if (!is_last_segment || !is_last_line_in_segment) && needs_newline {
                    new_segments.push(OutputSegment {
                        buf: prefix.to_owned(),
                        style: OutputStyle::default(),
                    });
                }
            }
        }

        new_segments.shrink_to_fit();

        new_segments
    }
}

#[cfg(feature = "color")]
pub use with_color::*;

#[cfg(feature = "fmt")]
mod with_fmt {
    use std::borrow::Cow;

    /// Return the length of a stringified integer.
    pub fn int_len(n: usize, base: u32) -> u32 {
        let mut power = base as usize;
        let mut count = 1;
        while n >= power {
            count += 1;
            if let Some(new_power) = power.checked_mul(base as usize) {
                power = new_power;
            } else {
                break;
            }
        }
        count
    }

    /// Return the amount of padding necessary to align an integer.
    pub fn pad_int(n: usize, longest: usize, base: u32) -> Cow<'static, str> {
        super::whitespace((int_len(longest, base) - int_len(n, base)) as usize)
    }
}

#[cfg(feature = "fmt")]
pub use with_fmt::*;

/// Indent each line by the given number of spaces.
#[cfg(any(not(feature = "color"), test))]
pub fn indent<'a>(s: &'a str, prefix: &'a str, hanging: bool) -> Cow<'a, str> {
    if s.is_empty() || prefix.is_empty() {
        return Cow::Borrowed(s);
    }

    // We know that we'll need more than `s.len()` bytes for the output, but we don't know exactly
    // how many without counting LF characters, which is expensive.
    let mut result = String::with_capacity(s.len() * 2);

    for (i, line) in s.lines().enumerate() {
        if !hanging || i != 0 {
            result.push_str(prefix.as_ref());
        }

        result.push_str(line);
        result.push('\n');
    }

    // If the input doesn't end with a newline, don't add one.
    if !s.ends_with('\n') {
        result.truncate(result.len() - 1);
    }

    // We most likely over-allocated.
    result.shrink_to_fit();

    Cow::Owned(result)
}

/// Return a string of whitespace of the given length.
///
/// You can use this for indenting text with whitespace via [`Formatter::indented`] and
/// [`FormattedOutput::indented`].
///
/// [`Formatter::indented`]: crate::core::Formatter::indented
/// [`FormattedOutput::indented`]: crate::core::FormattedOutput::indented
pub fn whitespace(spaces: usize) -> Cow<'static, str> {
    match spaces {
        i if i < PREFIX_CACHE.len() => Cow::Borrowed(&PREFIX_CACHE[..i]),
        i => Cow::Owned(" ".repeat(i)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_when_the_indent_len_is_zero() {
        let input = "line 1\nline 2\n line 3\n";
        let expected = Cow::Borrowed(input);
        assert_eq!(indent(input, "", false), expected);
    }

    #[test]
    fn indent_when_the_indent_len_is_non_zero() {
        let input = " line 1\n  line 2\n   line 3\n";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n    line 2\n     line 3\n"));
        assert_eq!(indent(input, "  ", false), expected);
    }

    #[test]
    fn indent_when_there_is_no_trailing_newline() {
        let input = " line 1\n line 2\n line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n   line 2\n   line 3"));
        assert_eq!(indent(input, "  ", false), expected);
    }
}
