use std::borrow::Cow;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_when_the_indent_len_is_zero() {
        let actual = "line 1\nline 2\n line 3";
        let expected = Cow::Borrowed(actual);
        assert_eq!(indent(actual, 0), expected);
    }

    #[test]
    fn indent_when_the_indent_len_is_non_zero() {
        let actual = " line 1\n  line 2\n   line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n    line 2\n     line 3\n"));
        assert_eq!(indent(&actual, 2), expected);
    }

    #[test]
    fn indent_when_there_is_no_trailing_newline() {
        let actual = " line 1\n line 2\n line 3";
        let expected = Cow::<'_, str>::Owned(String::from("   line 1\n   line 2\n   line 3"));
        assert_eq!(indent(&actual, 2), expected);
    }
}
