use crate::parse::{
    lex::err::{LexErr, LexErrTag},
    position::DocumentPosition,
    token::{Token, TokenTag},
};

pub struct Scanner<'a> {
    filename: &'a str,
    content: &'a str,
    start_position: DocumentPosition,
    current_position: DocumentPosition,
}

impl<'a> Scanner<'a> {
    pub fn new(filename: &'a str, content: &'a str) -> Self {
        Scanner {
            filename,
            content,
            start_position: DocumentPosition::default(),
            current_position: DocumentPosition::default(),
        }
    }

    fn newline(&mut self) {
        self.current_position.newline();
    }

    fn at_end(&self) -> bool {
        self.current_position.offset == self.content.len()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        return self
            .content
            .chars()
            .nth(self.current_position.offset)
            .expect("there to be a character");
    }

    fn peek_next(&self) -> char {
        if self.current_position.offset + 1 >= self.content.len() {
            return '\0';
        };

        return self
            .content
            .chars()
            .nth(self.current_position.offset + 1)
            .expect("there to be a character");
    }

    fn advance(&mut self) -> char {
        let char = self
            .content
            .chars()
            .nth(self.current_position.offset)
            .expect("there to be a character");

        self.current_position.advance();
        char
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.current_position.advance();
        return true;
    }

    fn mk_token(&mut self, tag: TokenTag) -> Result<Token, LexErr> {
        let lexeme =
            self.content[self.start_position.offset..self.current_position.offset].to_owned();
        let token = Token::new(tag, lexeme, self.start_position.clone());

        self.start_position.set(&self.current_position);

        Ok(token)
    }

    fn mk_err(&self, tag: LexErrTag) -> Result<Token, LexErr> {
        Err(LexErr::new(
            tag,
            self.filename.to_owned(),
            self.start_position.clone(),
        ))
    }

    fn string(&mut self) -> Result<Token, LexErr> {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.newline();
            }

            if self.peek() == '\\' {
                self.advance();
                if self.at_end() {
                    break;
                }
            }

            self.advance();
        }

        if self.at_end() {
            return self.mk_err(LexErrTag::UnterminatedString);
        }

        // The closing ".
        self.advance();
        self.mk_token(TokenTag::String)
    }

    fn multiline(&mut self) -> Result<Token, LexErr> {
        let mut count = 0;
        loop {
            if self.at_end() {
                break;
            }

            if self.peek() == '*' && self.peek_next() == '/' {
                if count > 0 { count -= 1 } else { break }
            }

            if self.peek() == '/' && self.peek_next() == '*' {
                count += 1;
            }

            self.advance();
        }

        if self.at_end() {
            return self.mk_err(LexErrTag::UnterminatedComment);
        }

        self.advance();
        self.advance();
        self.mk_token(TokenTag::InlineComment)
    }

    fn number(&mut self) -> Result<Token, LexErr> {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.mk_token(TokenTag::Number)
    }

    fn identifier(&mut self) -> Result<Token, LexErr> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let tag = match &self.content[self.start_position.offset..self.current_position.offset] {
            "and" => TokenTag::And,
            "class" => TokenTag::Class,
            "else" => TokenTag::Else,
            "false" => TokenTag::False,
            "for" => TokenTag::For,
            "fun" => TokenTag::Fun,
            "if" => TokenTag::If,
            "nil" => TokenTag::Nil,
            "or" => TokenTag::Or,
            "print" => TokenTag::Print,
            "return" => TokenTag::Return,
            "super" => TokenTag::Super,
            "this" => TokenTag::This,
            "true" => TokenTag::True,
            "var" => TokenTag::Var,
            "while" => TokenTag::While,
            _ => TokenTag::Identifier,
        };

        self.mk_token(tag)
    }

    fn scan_token(&mut self) -> Result<Token, LexErr> {
        match self.advance() {
            '(' => self.mk_token(TokenTag::LeftParen),
            ')' => self.mk_token(TokenTag::RightParen),
            '{' => self.mk_token(TokenTag::LeftBrace),
            '}' => self.mk_token(TokenTag::RightBrace),
            ',' => self.mk_token(TokenTag::Comma),
            '.' => self.mk_token(TokenTag::Dot),
            '-' => self.mk_token(TokenTag::Minus),
            '+' => self.mk_token(TokenTag::Plus),
            ';' => self.mk_token(TokenTag::SemiColon),
            '*' => self.mk_token(TokenTag::Star),
            '!' => {
                if self.matches('=') {
                    self.mk_token(TokenTag::BangEqual)
                } else {
                    self.mk_token(TokenTag::Bang)
                }
            }

            '=' => {
                if self.matches('=') {
                    self.mk_token(TokenTag::EqualEqual)
                } else {
                    self.mk_token(TokenTag::Equal)
                }
            }

            '<' => {
                if self.matches('=') {
                    self.mk_token(TokenTag::LessEqual)
                } else {
                    self.mk_token(TokenTag::Less)
                }
            }

            '>' => {
                if self.matches('=') {
                    self.mk_token(TokenTag::GreaterEqual)
                } else {
                    self.mk_token(TokenTag::Greater)
                }
            }

            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                    self.mk_token(TokenTag::Comment)
                } else if self.matches('*') {
                    self.multiline()
                } else {
                    self.mk_token(TokenTag::Slash)
                }
            }

            '"' => self.string(),

            c => {
                if c.is_whitespace() {
                    panic!("All whitespace should get eaten by eat_whitespace")
                }

                if c.is_numeric() {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    return self.mk_err(LexErrTag::UnexpectedCharacter);
                }
            }
        }
    }

    fn eat_whitespace(&mut self) {
        while !self.at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.newline();
            }
            self.advance();
        }
        self.start_position.set(&self.current_position);
    }

    pub fn scan(&mut self) -> (Vec<Token>, Vec<LexErr>) {
        let mut tokens = vec![];
        let mut errs = vec![];

        while !self.at_end() {
            self.eat_whitespace();

            if self.at_end() {
                break;
            }

            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(e) => errs.push(e),
            }
        }

        tokens.push(Token::eof(self.current_position.clone()));

        (tokens, errs)
    }
}
