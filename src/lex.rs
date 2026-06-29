use crate::lex::ParseErr::UnterminatedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenTag {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,
    Comment,
    InlineComment,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub enum ParseErr {
    UnterminatedString,
    UnexpectedCharacter,
}

#[derive(Clone, Debug)]
pub struct Token {
    tag: TokenTag,
    lexeme: String,
    line: usize,
    column: usize,
}

impl Token {
    fn new(tag: TokenTag, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            tag,
            lexeme,
            line,
            column,
        }
    }
}

pub struct Scanner<'a> {
    content: &'a str,
    start: usize,
    pos: usize,
    line: usize,
    start_col: usize,
    current_col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Scanner {
            content,
            start: 0,
            pos: 0,
            line: 1,
            start_col: 1,
            current_col: 1,
        }
    }

    fn newline(&mut self) {
        self.line += 1;
        self.current_col = 1;
    }

    fn at_end(&self) -> bool {
        self.pos == self.content.len()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        return self
            .content
            .chars()
            .nth(self.pos)
            .expect("there to be a character");
    }

    fn peek_next(&self) -> char {
        if self.pos + 1 >= self.content.len() {
            return '\0';
        };

        return self
            .content
            .chars()
            .nth(self.pos + 1)
            .expect("there to be a character");
    }

    fn advance(&mut self) -> char {
        let char = self
            .content
            .chars()
            .nth(self.pos)
            .expect("there to be a character");

        self.pos += 1;
        self.current_col += 1;
        char
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.content.chars().nth(self.pos).unwrap_or('\0') != expected {
            return false;
        }

        self.pos += 1;
        self.current_col += 1;
        return true;
    }

    fn mk_token(&mut self, tag: TokenTag) -> Result<Token, ParseErr> {
        let lexeme = self.content[self.start..self.pos].to_owned();
        let token = Token::new(tag, lexeme, self.line, self.start_col);
        self.start = self.pos;
        self.start_col = self.current_col;

        Ok(token)
    }

    fn string(&mut self) -> Result<Token, ParseErr> {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.newline();
            }

            self.advance();
        }

        if self.at_end() {
            return Err(UnterminatedString);
        }

        // The closing ".
        self.advance();
        self.mk_token(TokenTag::String)
    }

    fn number(&mut self) -> Result<Token, ParseErr> {
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

    fn identifier(&mut self) -> Result<Token, ParseErr> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let tag = match &self.content[self.start..self.pos] {
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

    fn scan_token(&mut self) -> Result<Token, ParseErr> {
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
                    while self.peek() != '*' && self.peek_next() != '/' && !self.at_end() {
                        self.advance();
                    }
                    self.advance();
                    self.advance();
                    self.mk_token(TokenTag::InlineComment)
                } else {
                    self.mk_token(TokenTag::Slash)
                }
            }

            '"' => self.string(),

            c => {
                if c.is_numeric() {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else if c.is_whitespace() {
                    panic!("All whitespace should get eaten by eat_whitespace")
                } else {
                    return Err(ParseErr::UnexpectedCharacter);
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
        self.start = self.pos;
        self.start_col = self.current_col;
    }

    pub fn scan(&mut self) -> (Vec<Token>, Vec<ParseErr>) {
        let mut tokens = vec![];
        let mut errs = vec![];

        while !self.at_end() {
            self.start = self.pos;
            self.eat_whitespace();

            if self.at_end() {
                break;
            }

            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(e) => errs.push(e),
            }
        }

        tokens.push(Token {
            tag: TokenTag::EOF,
            lexeme: "".to_owned(),
            line: self.line,
            column: self.start_col,
        });

        (tokens, errs)
    }
}
