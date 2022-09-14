use std::borrow::Cow;

const PREFIX_CACHE: &'static str =
    "                                                                ";

#[cfg(feature = "color")]
mod with_color {
    use super::get_prefix;
    use std::ops::Range;

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

    struct Line<'a> {
        s: &'a str,
        has_trailing: bool,
    }

    impl<'a> Line<'a> {
        pub fn as_str(&self) -> &str {
            self.s
        }

        pub fn has_trailing(&self) -> bool {
            self.has_trailing
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum LinebreakState {
        BeforeFirst,
        AfterLast,
        Between,
    }

    struct Lines<'a> {
        s: &'a str,
        pos: usize,
        state: LinebreakState,
    }

    impl<'a> Iterator for Lines<'a> {
        type Item = Line<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            match (self.state, find_linebreak(&self.s[self.pos..])) {
                (LinebreakState::BeforeFirst, Some(next_linebreak)) => {
                    let range = 0..next_linebreak.start;
                    self.pos = next_linebreak.end;
                    self.state = LinebreakState::Between;

                    Some(Line {
                        s: &self.s[range],
                        has_trailing: next_linebreak.end == self.s.len(),
                    })
                }
                (LinebreakState::BeforeFirst, None) => {
                    self.state = LinebreakState::AfterLast;

                    Some(Line {
                        s: &self.s,
                        has_trailing: false,
                    })
                }
                (LinebreakState::Between, None) => {
                    self.state = LinebreakState::AfterLast;

                    Some(Line {
                        s: &self.s[self.pos..],
                        has_trailing: false,
                    })
                }
                (LinebreakState::Between, Some(next_linebreak)) => {
                    let range = self.pos..next_linebreak.start;
                    self.pos = next_linebreak.end;

                    Some(Line {
                        s: &self.s[range],
                        has_trailing: next_linebreak.end == self.s.len(),
                    })
                }
                _ => None,
            }
        }
    }

    fn iter_lines<'a>(s: &'a str) -> Lines {
        Lines {
            s,
            pos: 0,
            state: LinebreakState::BeforeFirst,
        }
    }

    pub fn indent_vec<'a>(
        segments: impl IntoIterator<Item = String>,
        spaces: u32,
        hanging: bool,
    ) -> Vec<String> {
        if spaces == 0 {
            return segments.into_iter().collect();
        }

        let prefix = get_prefix(spaces as usize);

        let mut new_segments = Vec::new();
        let mut current_line = String::new();
        let mut is_first_line = true;

        for segment in segments.into_iter() {
            if segment.is_empty() {
                new_segments.push(segment);
                continue;
            }

            let mut new_segment = String::new();

            for line in iter_lines(&segment) {
                current_line.push_str(line.as_str());

                if line.has_trailing() && !current_line.is_empty() {
                    if !hanging || !is_first_line {
                        new_segment.push_str(&prefix);
                    }

                    is_first_line = false;

                    new_segment.push_str(&current_line);
                    new_segment.push('\n');

                    current_line.clear();
                }
            }

            new_segments.push(new_segment);
        }

        new_segments
    }
}

#[cfg(feature = "color")]
pub(super) use with_color::*;

fn get_prefix<'a>(spaces: usize) -> Cow<'a, str> {
    match spaces as usize {
        i if i < PREFIX_CACHE.len() => Cow::Borrowed(&PREFIX_CACHE[..i]),
        i => Cow::Owned(" ".repeat(i as usize)),
    }
}

/// Indent each line by the given number of spaces.
pub(super) fn indent<'a>(s: &'a str, spaces: u32, hanging: bool) -> Cow<'a, str> {
    if s.is_empty() || spaces == 0 {
        return Cow::Borrowed(s);
    }

    let prefix = get_prefix(spaces as usize);

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
