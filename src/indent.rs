use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IndentWriter<T> {
    inner: T,
    indent: u32,
    prefix: Option<Cow<'static, str>>,
    newline: bool,
}

impl<T> IndentWriter<T> {
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

    pub fn new(inner: T) -> Self {
        Self {
            inner,
            indent: 0,
            prefix: None,
            newline: true,
        }
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
            _ => self.prefix = Some(Cow::Owned(" ".repeat(indent as usize))),
        }
    }
}

impl<T> IndentWriter<T>
where
    T: AsRef<str>,
{
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }
}

impl<T> AsRef<str> for IndentWriter<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl<T> fmt::Write for IndentWriter<T>
where
    T: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() == 0 {
            return Ok(());
        }

        if let Some(prefix) = &self.prefix {
            for (i, line) in s.lines().enumerate() {
                if i > 0 {
                    self.inner.write_char('\n')?;
                }

                if !line.is_empty() {
                    self.inner.write_str(&prefix)?;
                }

                self.inner.write_str(line)?;
            }

            self.newline = s.ends_with('\n');
            if self.newline {
                self.inner.write_char('\n')?;
            }

            Ok(())
        } else {
            self.inner.write_str(s)
        }
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        match &self.prefix {
            Some(prefix) if self.newline => {
                self.inner.write_str(&prefix)?;
            }
            _ => {}
        }

        self.newline = c == '\n';
        self.inner.write_char(c)
    }
}
