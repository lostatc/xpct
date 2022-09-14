use std::borrow::Cow;
use std::cmp;

const PREFIX_CACHE: &'static str =
    "                                                                ";

/// Indent each line by the given number of spaces.
pub(super) fn indent<'a>(s: &'a str, spaces: u32) -> Cow<'a, str> {
    let prefix = match spaces as usize {
        i if i == 0 => return Cow::Borrowed(s),
        i if i < PREFIX_CACHE.len() => Cow::Borrowed(&PREFIX_CACHE[..i]),
        i => Cow::Owned(" ".repeat(i as usize)),
    };

    if s.is_empty() {
        return Cow::Borrowed("");
    }

    // We know that we'll need more than `s.len()` bytes for the output, but we don't know exactly
    // how many without counting LF characters, which is expensive.
    let mut result = String::with_capacity(s.len() * 2);

    for line in s.lines() {
        result.push_str(prefix.as_ref());
        result.push_str(line);
        result.push('\n');
    }

    // If the input doesn't end with a newline, don't add one.
    if !s.ends_with('\n') {
        result.truncate(result.len() - 1);
    }

    // Unless the string was small and the amount of indentation was large, we most likely
    // over-allocated.
    result.shrink_to_fit();

    Cow::Owned(result)
}

/// Remove common leading whitespace from each line.
///
/// This function will look at each non-empty line and remove the maximum amount of whitespace that
/// can be removed from all lines.
pub(super) fn dedent<'a>(s: &'a str) -> Cow<'a, str> {
    let mut prefix_len = s.len();
    let mut num_lines = 0;

    for line in s.lines() {
        // We need this to calculate the capacity of the output buffer.
        num_lines += 1;

        for (index, c) in line.char_indices() {
            if !c.is_whitespace() {
                prefix_len = cmp::min(index, prefix_len);
                break;
            }
        }
    }

    // There is no indentation to dedent, so there's no need to allocate anything.
    if prefix_len == s.len() {
        return Cow::Borrowed(s);
    }

    // We can calculate the exact capacity we need.
    let mut result = String::with_capacity(s.len() - (num_lines * prefix_len));

    for line in s.lines() {
        if line.len() >= prefix_len {
            result.push_str(&line[prefix_len..])
        } else {
            // This is a blank line whose length is less than the prefix.
            result.push_str(line)
        }

        result.push('\n');
    }

    // If the input doesn't end with a newline, don't add one.
    if !s.ends_with('\n') {
        result.truncate(result.len() - 1);
    }

    Cow::Owned(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indented_string(levels: &[usize]) -> String {
        let mut result = String::new();

        for (index, level) in levels.iter().enumerate() {
            for _ in 0..*level {
                result.push(' ');
            }
            result.push_str(&format!("line {}", index));
            result.push('\n');
        }

        result
    }

    #[test]
    fn indent_when_the_indent_len_is_zero() {
        let actual = "line 1\nline 2\n line 3";
        let expected = Cow::Borrowed(actual);
        assert_eq!(indent(actual, 0), expected);
    }

    #[test]
    fn indent_when_the_indent_len_is_non_zero() {
        let actual = indented_string(&[1, 2, 3]);
        let expected = Cow::<'_, str>::Owned(indented_string(&[5, 6, 7]));
        assert_eq!(indent(&actual, 4), expected);
    }

    #[test]
    fn indent_when_there_is_no_trailing_newline() {
        let actual = " line 1\n line 2\n line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n   line 2\n   line 3"));
        assert_eq!(indent(&actual, 2), expected);
    }

    #[test]
    fn dedent_when_there_is_no_indent() {
        let actual = indented_string(&[0, 0, 0]);
        let expected = Cow::Borrowed(&actual);
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_indent_level_increases() {
        let actual = indented_string(&[1, 2, 3]);
        let expected = Cow::<'_, str>::Owned(indented_string(&[0, 1, 2]));
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_indent_level_decreases() {
        let actual = indented_string(&[3, 2, 1]);
        let expected = Cow::<'_, str>::Owned(indented_string(&[2, 1, 0]));
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_indent_level_decreases_then_increases() {
        let actual = indented_string(&[2, 1, 3]);
        let expected = Cow::<'_, str>::Owned(indented_string(&[1, 0, 2]));
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_indent_level_increases_then_decreases() {
        let actual = indented_string(&[1, 3, 2]);
        let expected = Cow::<'_, str>::Owned(indented_string(&[0, 2, 1]));
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_there_is_a_blank_line() {
        let actual = " line 1\n\n line 3\n";
        let expected = Cow::<'_, str>::Owned(String::from("line 1\n\nline 3\n"));
        assert_eq!(dedent(&actual), expected);
    }

    #[test]
    fn dedent_when_there_is_no_trailing_newline() {
        let actual = " line 1\n\n line 3";
        let expected = Cow::<'_, str>::Owned(String::from("line 1\n\nline 3"));
        assert_eq!(dedent(&actual), expected);
    }
}
