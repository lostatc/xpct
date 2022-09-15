use std::borrow::Cow;

const PREFIX_CACHE: &'static str =
    "                                                                ";

#[cfg(feature = "color")]
mod with_color {
    use super::whitespace;
    use std::ops::Range;

    // Find the first LF or CRLF linebreak in the given string.
    //
    // This returns the bytes range of the linebreak characters, not the byte range of the line
    // between the linebreaks.
    //
    // If no linebreak was found, this returns `None`.
    //
    // ```
    // assert_eq!(find_linebreak("abc\r\ndef"), Some(3..5));
    // ```
    fn find_linebreak<'a>(s: &'a str) -> Option<Range<usize>> {
        let mut chars = s.char_indices();

        loop {
            match chars.next() {
                Some((start, '\n')) => {
                    if let Some((_, '\r')) = chars.next() {
                        return Some(start..start + 2);
                    } else {
                        return Some(start..start + 1);
                    }
                }
                None => break,
                _ => {}
            }
        }

        None
    }

    pub fn indent_vec<'a>(
        segments: impl IntoIterator<Item = String>,
        spaces: u32,
        hanging: bool,
    ) -> Vec<String> {
        if spaces == 0 {
            return segments.into_iter().collect();
        }

        let prefix = whitespace(spaces as usize);

        let mut new_segments = Vec::new();

        // Whether the next write should be adding indentation.
        let mut needs_indented = !hanging;

        for segment in segments.into_iter() {
            if segment.is_empty() {
                // This segment is empty, so no need for indentation.
                new_segments.push(segment);
                continue;
            }

            let mut pos = 0;

            // We know that we'll need more than `segment.len()` bytes for the output, but we don't
            // know exactly how many yet.
            let mut new_segment = String::with_capacity(segment.len() * 2);

            // Iterate over linebreaks (LF or CRLF) in the segment.
            while let Some(linebreak) = find_linebreak(&segment[pos..]) {
                if pos == linebreak.start {
                    // This is a blank line and should not be indented.
                    new_segment.push('\n');
                } else {
                    if needs_indented {
                        new_segment.push_str(&prefix);
                    }

                    new_segment.push_str(&segment[pos..pos + linebreak.start]);
                    new_segment.push('\n');
                }

                needs_indented = true;
                pos += linebreak.end;
            }

            if pos <= segment.len() - 1 {
                // There are characters between the last linebreak and the end of the segment.
                if needs_indented {
                    new_segment.push_str(&prefix);
                    needs_indented = false;
                }

                new_segment.push_str(&segment[pos..]);
            }

            // We most likely over-allocated.
            new_segment.shrink_to_fit();

            new_segments.push(new_segment);
        }

        new_segments
    }
}

#[cfg(feature = "color")]
pub use with_color::*;

#[cfg(feature = "fmt")]
mod with_fmt {
    use std::borrow::Cow;

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

    pub fn pad_int(n: usize, longest: usize, base: u32) -> Cow<'static, str> {
        super::whitespace((int_len(longest, base) - int_len(n, base)) as usize)
    }
}

#[cfg(feature = "fmt")]
pub use with_fmt::*;

pub fn whitespace<'a>(spaces: usize) -> Cow<'a, str> {
    match spaces as usize {
        i if i < PREFIX_CACHE.len() => Cow::Borrowed(&PREFIX_CACHE[..i]),
        i => Cow::Owned(" ".repeat(i as usize)),
    }
}

/// Indent each line by the given number of spaces.
pub fn indent<'a>(s: &'a str, spaces: u32, hanging: bool) -> Cow<'a, str> {
    if s.is_empty() || spaces == 0 {
        return Cow::Borrowed(s);
    }

    let prefix = whitespace(spaces as usize);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_when_the_indent_len_is_zero() {
        let actual = "line 1\nline 2\n line 3";
        let expected = Cow::Borrowed(actual);
        assert_eq!(indent(actual, 0, false), expected);
    }

    #[test]
    fn indent_when_the_indent_len_is_non_zero() {
        let actual = " line 1\n  line 2\n   line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n    line 2\n     line 3\n"));
        assert_eq!(indent(&actual, 2, false), expected);
    }

    #[test]
    fn indent_when_there_is_no_trailing_newline() {
        let actual = " line 1\n line 2\n line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n   line 2\n   line 3"));
        assert_eq!(indent(&actual, 2, false), expected);
    }
}
