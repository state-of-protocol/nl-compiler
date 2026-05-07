pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            source: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        match self.source.get(self.pos) {
            Some(ch) => {
                self.pos += 1;
                if *ch == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(*ch)
            }
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_ident_char(ch: char, first: bool) -> bool {
        if first {
            ch.is_alphabetic() || ch == '_'
        } else {
            ch.is_alphanumeric() || ch == '_'
        }
    }

    fn read_identifier(&mut self, first_char: char) -> super::TokenKind {
        let mut ident = String::new();
        ident.push(first_char);
        while let Some(ch) = self.peek() {
            if Self::is_ident_char(ch, false) {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "fn" => super::TokenKind::Fn,
            "biar" => super::TokenKind::Biar,
            "jika" => super::TokenKind::Jika,
            "lain" => super::TokenKind::Lain,
            "untuk" => super::TokenKind::Untuk,
            "sementara" => super::TokenKind::Sementara,
            _ => super::TokenKind::Ident(ident),
        }
    }

    fn read_integer(&mut self, first_digit: char) -> Result<i64, String> {
        let mut number = String::new();
        number.push(first_digit);
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        number.parse::<i64>().map_err(|_| {
            format!("Ralat lekser pada {}:{} - literal integer tidak sah '{}'",
                    self.line, self.col, number)
        })
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.advance(); // buka petikan
        let mut content = String::new();
        loop {
            match self.advance() {
                Some('"') => break,
                Some('\n') => {
                    return Err(format!(
                        "Ralat lekser pada {}:{} - rentetan tidak tertutup",
                        self.line, self.col
                    ));
                }
                Some(ch) => content.push(ch),
                None => {
                    return Err(format!(
                        "Ralat lekser pada {}:{} - akhir fail dalam rentetan",
                        self.line, self.col
                    ));
                }
            }
        }
        Ok(content)
    }

    pub fn tokenize(&mut self) -> Result<Vec<super::Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            let line = self.line;
            let col = self.col;

            match self.peek() {
                None => break,
                Some(ch) => {
                    if Self::is_ident_char(ch, true) {
                        self.advance();
                        let kind = self.read_identifier(ch);
                        tokens.push(super::Token::new(kind, line, col));
                    } else if ch.is_ascii_digit() {
                        self.advance();
                        let val = self.read_integer(ch)?;
                        tokens.push(super::Token::new(super::TokenKind::Int(val), line, col));
                    } else {
                        let kind = match ch {
                            '+' => { self.advance(); super::TokenKind::Plus }
                            '-' => { self.advance(); super::TokenKind::Minus }
                            '*' => { self.advance(); super::TokenKind::Star }
                            '/' => { self.advance(); super::TokenKind::Slash }
                            '(' => { self.advance(); super::TokenKind::LParen }
                            ')' => { self.advance(); super::TokenKind::RParen }
                            '{' => { self.advance(); super::TokenKind::LBrace }
                            '}' => { self.advance(); super::TokenKind::RBrace }
                            ',' => { self.advance(); super::TokenKind::Comma }
                            ';' => { self.advance(); super::TokenKind::Semicolon }
                            '.' => {
                                self.advance();
                                if self.peek() == Some('.') {
                                    self.advance();
                                    super::TokenKind::DotDot
                                } else {
                                    super::TokenKind::Dot
                                }
                            }
                            ':' => { self.advance(); super::TokenKind::Colon }
                            '=' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    super::TokenKind::EqualEqual
                                } else {
                                    super::TokenKind::Assign
                                }
                            }
                            '<' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    super::TokenKind::LessEqual
                                } else {
                                    super::TokenKind::Less
                                }
                            }
                            '>' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    super::TokenKind::GreaterEqual
                                } else {
                                    super::TokenKind::Greater
                                }
                            }
                            '!' => {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    super::TokenKind::NotEqual
                                } else {
                                    return Err(format!(
                                        "Ralat lekser pada {}:{} - aksara tidak dijangka '!'",
                                        line, col
                                    ));
                                }
                            }
                            '"' => {
                                let s = self.read_string()?;
                                super::TokenKind::String(s)
                            }
                            _ => {
                                return Err(format!(
                                    "Ralat lekser pada {}:{} - aksara tidak dijangka '{}'",
                                    line, col, ch
                                ));
                            }
                        };
                        tokens.push(super::Token::new(kind, line, col));
                    }
                }
            }
        }
        tokens.push(super::Token::new(super::TokenKind::Eof, self.line, self.col));
        Ok(tokens)
    }
}
