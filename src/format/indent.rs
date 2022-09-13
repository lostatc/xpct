use std::borrow::Cow;

const PREFIX_CACHE: [&'static str; 32] = [
    " ",
    "  ",
    "   ",
    "    ",
    "     ",
    "      ",
    "       ",
    "        ",
    "         ",
    "          ",
    "           ",
    "            ",
    "             ",
    "              ",
    "               ",
    "                ",
    "                 ",
    "                  ",
    "                   ",
    "                    ",
    "                     ",
    "                      ",
    "                       ",
    "                        ",
    "                         ",
    "                          ",
    "                           ",
    "                            ",
    "                             ",
    "                              ",
    "                               ",
    "                                ",
];

pub(super) fn indent<'a>(s: &'a str, spaces: u32) -> Option<String> {
    // We know that we'll need more than `s.len()` bytes for the output, but we don't know exactly
    // how many without counting LF characters, which is expensive.
    let mut result = String::with_capacity(s.len() * 2);

    let prefix = match spaces as usize {
        i if i == 0 => return None,
        i if i < PREFIX_CACHE.len() => Cow::Borrowed(PREFIX_CACHE[i - 1]),
        i => Cow::Owned(" ".repeat(i as usize)),
    };

    for line in s.lines() {
        result.push_str(prefix.as_ref());
        result.push_str(line);
    }

    result.shrink_to_fit();

    Some(result)
}
