# toml-rs

### all

It is a lib, so the lib.rs is all mod.

- datetime.rs

- de.rs

- lib.rs

- macros.rs

- map.rs

- ser.r

- spanned.rs

- tokens.rs

- value.rs



```toml
# Cargo.toml

[package]
name = "toml"
version = "0.5.5"
authors = ["Alex Crichton <alex@alexcrichton.com>"]
license = "MIT/Apache-2.0"
readme = "README.md"
keywords = ["encoding"]
repository = "https://github.com/alexcrichton/toml-rs"
homepage = "https://github.com/alexcrichton/toml-rs"
documentation = "https://docs.rs/toml"
description = """
A native Rust encoder and decoder of TOML-formatted files and streams. Provides
implementations of the standard Serialize/Deserialize traits for TOML data to
facilitate deserializing and serializing Rust structures.
"""
categories = ["config", "encoding", "parser-implementations"]
edition = "2018"

[workspace]
members = ['test-suite']

[dependencies]
serde = "1.0.97"
indexmap = { version = "1.0", optional = true }

[dev-dependencies]
serde_derive = "1.0"
serde_json = "1.0"

[features]
default = []

# Use indexmap rather than BTreeMap as the map type of toml::Value.
# This allows data to be read into a Value and written back to a TOML string
# while preserving the order of map keys in the input.
preserve_order = ["indexmap"]
```

### dependencies
```toml
serde_derive = "1.0"
serde_json = "1.0"
```
***next to read is serde***

### datetime.rs

- std.fmt
  fmt::Formatter<'_>

### tokens.rs

```rust
impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        let mut t = Tokenizer {
            input,
            chars: CrlfFold {
                chars: input.char_indices(),
            },
        };
        // Eat utf-8 BOM
        t.eatc('\u{feff}');
        t
    }

    pub fn next(&mut self) -> Result<Option<(Span, Token<'a>)>, Error> {
        let (start, token) = match self.one() {
            Some((start, '\n')) => (start, Newline),
            Some((start, ' ')) => (start, self.whitespace_token(start)),
            Some((start, '\t')) => (start, self.whitespace_token(start)),
            Some((start, '#')) => (start, self.comment_token(start)),
            Some((start, '=')) => (start, Equals),
            Some((start, '.')) => (start, Period),
            Some((start, ',')) => (start, Comma),
            Some((start, ':')) => (start, Colon),
            Some((start, '+')) => (start, Plus),
            Some((start, '{')) => (start, LeftBrace),
            Some((start, '}')) => (start, RightBrace),
            Some((start, '[')) => (start, LeftBracket),
            Some((start, ']')) => (start, RightBracket),
            Some((start, '\'')) => {
                return self
                    .literal_string(start)
                    .map(|t| Some((self.step_span(start), t)))
            }
            Some((start, '"')) => {
                return self
                    .basic_string(start)
                    .map(|t| Some((self.step_span(start), t)))
            }
            Some((start, ch)) if is_keylike(ch) => (start, self.keylike(start)),

            Some((start, ch)) => return Err(Error::Unexpected(start, ch)),
            None => return Ok(None),
        };

        let span = self.step_span(start);
        Ok(Some((span, token)))
    }

    pub fn peek(&mut self) -> Result<Option<(Span, Token<'a>)>, Error> {
        self.clone().next()
    }

    pub fn eat(&mut self, expected: Token<'a>) -> Result<bool, Error> {
        self.eat_spanned(expected).map(|s| s.is_some())
    }

    /// Eat a value, returning it's span if it was consumed.
    pub fn eat_spanned(&mut self, expected: Token<'a>) -> Result<Option<Span>, Error> {
        let span = match self.peek()? {
            Some((span, ref found)) if expected == *found => span,
            Some(_) => return Ok(None),
            None => return Ok(None),
        };

        drop(self.next());
        Ok(Some(span))
    }

    pub fn expect(&mut self, expected: Token<'a>) -> Result<(), Error> {
        // ignore span
        let _ = self.expect_spanned(expected)?;
        Ok(())
    }

    /// Expect the given token returning its span.
    pub fn expect_spanned(&mut self, expected: Token<'a>) -> Result<Span, Error> {
        let current = self.current();
        match self.next()? {
            Some((span, found)) => {
                if expected == found {
                    Ok(span)
                } else {
                    Err(Error::Wanted {
                        at: current,
                        expected: expected.describe(),
                        found: found.describe(),
                    })
                }
            }
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: expected.describe(),
                found: "eof",
            }),
        }
    }

    pub fn table_key(&mut self) -> Result<(Span, Cow<'a, str>), Error> {
        let current = self.current();
        match self.next()? {
            Some((span, Token::Keylike(k))) => Ok((span, k.into())),
            Some((
                span,
                Token::String {
                    src,
                    val,
                    multiline,
                },
            )) => {
                let offset = self.substr_offset(src);
                if multiline {
                    return Err(Error::MultilineStringKey(offset));
                }
                if val == "" {
                    return Err(Error::EmptyTableKey(offset));
                }
                match src.find('\n') {
                    None => Ok((span, val)),
                    Some(i) => Err(Error::NewlineInTableKey(offset + i)),
                }
            }
            Some((_, other)) => Err(Error::Wanted {
                at: current,
                expected: "a table key",
                found: other.describe(),
            }),
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: "a table key",
                found: "eof",
            }),
        }
    }

    pub fn eat_whitespace(&mut self) -> Result<(), Error> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        Ok(())
    }

    pub fn eat_comment(&mut self) -> Result<bool, Error> {
        if !self.eatc('#') {
            return Ok(false);
        }
        drop(self.comment_token(0));
        self.eat_newline_or_eof().map(|()| true)
    }

    pub fn eat_newline_or_eof(&mut self) -> Result<(), Error> {
        let current = self.current();
        match self.next()? {
            None | Some((_, Token::Newline)) => Ok(()),
            Some((_, other)) => Err(Error::Wanted {
                at: current,
                expected: "newline",
                found: other.describe(),
            }),
        }
    }

    pub fn skip_to_newline(&mut self) {
        loop {
            match self.one() {
                Some((_, '\n')) | None => break,
                _ => {}
            }
        }
    }

    fn eatc(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.one();
                true
            }
            _ => false,
        }
    }

    pub fn current(&mut self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    fn whitespace_token(&mut self, start: usize) -> Token<'a> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        Whitespace(&self.input[start..self.current()])
    }

    fn comment_token(&mut self, start: usize) -> Token<'a> {
        while let Some((_, ch)) = self.chars.clone().next() {
            if ch != '\t' && (ch < '\u{20}' || ch > '\u{10ffff}') {
                break;
            }
            self.one();
        }
        Comment(&self.input[start..self.current()])
    }

    fn read_string(
        &mut self,
        delim: char,
        start: usize,
        new_ch: &mut dyn FnMut(
            &mut Tokenizer<'_>,
            &mut MaybeString,
            bool,
            usize,
            char,
        ) -> Result<(), Error>,
    ) -> Result<Token<'a>, Error> {
        let mut multiline = false;
        if self.eatc(delim) {
            if self.eatc(delim) {
                multiline = true;
            } else {
                return Ok(String {
                    src: &self.input[start..start + 2],
                    val: Cow::Borrowed(""),
                    multiline: false,
                });
            }
        }
        let mut val = MaybeString::NotEscaped(self.current());
        let mut n = 0;
        'outer: loop {
            n += 1;
            match self.one() {
                Some((i, '\n')) => {
                    if multiline {
                        if self.input.as_bytes()[i] == b'\r' {
                            val.to_owned(&self.input[..i]);
                        }
                        if n == 1 {
                            val = MaybeString::NotEscaped(self.current());
                        } else {
                            val.push('\n');
                        }
                        continue;
                    } else {
                        return Err(Error::NewlineInString(i));
                    }
                }
                Some((i, ch)) if ch == delim => {
                    if multiline {
                        if !self.eatc(delim) {
                            val.push(delim);
                            continue 'outer;
                        }
                        if !self.eatc(delim) {
                            val.push(delim);
                            val.push(delim);
                            continue 'outer;
                        }
                    }
                    return Ok(String {
                        src: &self.input[start..self.current()],
                        val: val.into_cow(&self.input[..i]),
                        multiline,
                    });
                }
                Some((i, c)) => new_ch(self, &mut val, multiline, i, c)?,
                None => return Err(Error::UnterminatedString(start)),
            }
        }
    }

    fn literal_string(&mut self, start: usize) -> Result<Token<'a>, Error> {
        self.read_string('\'', start, &mut |_me, val, _multi, i, ch| {
            if ch == '\u{09}' || ('\u{20}' <= ch && ch <= '\u{10ffff}' && ch != '\u{7f}') {
                val.push(ch);
                Ok(())
            } else {
                Err(Error::InvalidCharInString(i, ch))
            }
        })
    }

    fn basic_string(&mut self, start: usize) -> Result<Token<'a>, Error> {
        self.read_string('"', start, &mut |me, val, multi, i, ch| match ch {
            '\\' => {
                val.to_owned(&me.input[..i]);
                match me.chars.next() {
                    Some((_, '"')) => val.push('"'),
                    Some((_, '\\')) => val.push('\\'),
                    Some((_, 'b')) => val.push('\u{8}'),
                    Some((_, 'f')) => val.push('\u{c}'),
                    Some((_, 'n')) => val.push('\n'),
                    Some((_, 'r')) => val.push('\r'),
                    Some((_, 't')) => val.push('\t'),
                    Some((i, c @ 'u')) | Some((i, c @ 'U')) => {
                        let len = if c == 'u' { 4 } else { 8 };
                        val.push(me.hex(start, i, len)?);
                    }
                    Some((i, c @ ' ')) | Some((i, c @ '\t')) | Some((i, c @ '\n')) if multi => {
                        if c != '\n' {
                            while let Some((_, ch)) = me.chars.clone().next() {
                                match ch {
                                    ' ' | '\t' => {
                                        me.chars.next();
                                        continue;
                                    }
                                    '\n' => {
                                        me.chars.next();
                                        break;
                                    }
                                    _ => return Err(Error::InvalidEscape(i, c)),
                                }
                            }
                        }
                        while let Some((_, ch)) = me.chars.clone().next() {
                            match ch {
                                ' ' | '\t' | '\n' => {
                                    me.chars.next();
                                }
                                _ => break,
                            }
                        }
                    }
                    Some((i, c)) => return Err(Error::InvalidEscape(i, c)),
                    None => return Err(Error::UnterminatedString(start)),
                }
                Ok(())
            }
            ch if ch == '\u{09}' || ('\u{20}' <= ch && ch <= '\u{10ffff}' && ch != '\u{7f}') => {
                val.push(ch);
                Ok(())
            }
            _ => Err(Error::InvalidCharInString(i, ch)),
        })
    }

    fn hex(&mut self, start: usize, i: usize, len: usize) -> Result<char, Error> {
        let mut buf = StdString::with_capacity(len);
        for _ in 0..len {
            match self.one() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_digit(16) => buf.push(ch),
                Some((i, ch)) => return Err(Error::InvalidHexEscape(i, ch)),
                None => return Err(Error::UnterminatedString(start)),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(Error::InvalidEscapeValue(i, val)),
        }
    }

    fn keylike(&mut self, start: usize) -> Token<'a> {
        while let Some((_, ch)) = self.peek_one() {
            if !is_keylike(ch) {
                break;
            }
            self.one();
        }
        Keylike(&self.input[start..self.current()])
    }

    pub fn substr_offset(&self, s: &'a str) -> usize {
        assert!(s.len() <= self.input.len());
        let a = self.input.as_ptr() as usize;
        let b = s.as_ptr() as usize;
        assert!(a <= b);
        b - a
    }

    /// Calculate the span of a single character.
    fn step_span(&mut self, start: usize) -> Span {
        let end = self
            .peek_one()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        Span { start, end }
    }

    /// Peek one char without consuming it.
    fn peek_one(&mut self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    /// Take one char.
    pub fn one(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
}
```

### de.rs

```
    fn table_header(&mut self) -> Result<Line<'a>, Error> {
        let start = self.tokens.current();
        self.expect(Token::LeftBracket)?;
        let array = self.eat(Token::LeftBracket)?;
        let ret = Header::new(self.tokens.clone(), array, self.require_newline_after_table);
        if self.require_newline_after_table {
            self.tokens.skip_to_newline();
        } else {
            loop {
                match self.next()? {
                    Some((_, Token::RightBracket)) => {
                        if array {
                            self.eat(Token::RightBracket)?;
                        }
                        break;
                    }
                    Some((_, Token::Newline)) | None => break,
                    _ => {}
                }
            }
            self.eat_whitespace()?;
        }
        Ok(Line::Table {
            at: start,
            header: ret,
            array,
        })
    }
```