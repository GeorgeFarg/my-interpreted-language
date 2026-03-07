pub struct Lexer<'a> {
    pub source_code: &'a str,
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            source_code: code,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source_code[self.pos..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.source_code.len()
    }

    pub fn lex_number(&mut self) -> &str {
        let start = self.pos - 1;

        while let Some(n) = self.peek() {
            if n.is_ascii_digit() {
                self.next();
            } else {
                break;
            }
        }
        &self.source_code[start..self.pos]
    }

    pub fn lex_word(&mut self) -> &str {
        let start = self.pos - 1;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                self.next();
            } else {
                break;
            }
        }
        &self.source_code[start..self.pos]
    }
}
