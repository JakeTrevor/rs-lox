use std::fmt::Debug;

use crate::parse::position::DocumentPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parse) enum TokenTag {
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
    Question,
    Colon,
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

#[derive(Clone, Debug)]
pub struct Token {
    pub tag: TokenTag,
    pub lexeme: String,
    pub position: DocumentPosition,
}

impl Token {
    pub(in crate::parse) fn new(tag: TokenTag, lexeme: String, position: DocumentPosition) -> Self {
        Token {
            tag,
            lexeme,
            position,
        }
    }

    pub(in crate::parse) fn eof(position: DocumentPosition) -> Self {
        Token {
            tag: TokenTag::EOF,
            lexeme: "".to_owned(),
            position,
        }
    }
}
