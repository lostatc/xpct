use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct IndentWriter {
    inner: String,
    indent: u32,
    prefix: Option<Cow<'static, str>>,
    newline: bool,
}

impl IndentWriter {
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

    pub fn new() -> Self {
        Self {
            inner: String::new(),
            indent: 0,
            prefix: None,
            newline: true,
        }
    }

    pub fn into_inner(self) -> String {
        self.inner
    }

    pub fn indent(&self) -> u32 {
        self.indent
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.indent = indent;
        match indent {
            i if i == 0 => self.prefix = None,
            i if i < Self::PREFIX_CACHE.len() as u32 => {
                self.prefix = Some(Cow::Borrowed(Self::PREFIX_CACHE[i as usize - 1]))
            }
            i => self.prefix = Some(Cow::Owned(" ".repeat(i as usize))),
        }
    }

    fn newline(&self) -> bool {
        if let Some(character) = self.inner.chars().rev().nth(0) {
            character == '\n'
        } else {
            true
        }
    }

    pub fn write_str(&mut self, s: &str) {
        if s.len() == 0 {
            return;
        }

        if let Some(prefix) = &self.prefix {
            for (i, line) in s.lines().enumerate() {
                if i > 0 {
                    self.inner.push('\n');
                }

                if !line.is_empty() && self.newline() {
                    self.inner.push_str(&prefix);
                }

                self.inner.push_str(line);
            }

            if self.newline() {
                self.inner.push('\n');
            }
        } else {
            self.inner.push_str(s);
        }
    }

    pub fn write_char(&mut self, c: char) {
        match &self.prefix {
            Some(prefix) if self.newline() => {
                self.inner.push_str(&prefix);
            }
            _ => {}
        }

        self.newline = c == '\n';
        self.inner.push(c)
    }
}

impl AsRef<str> for IndentWriter {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
