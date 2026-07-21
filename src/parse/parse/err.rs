use ariadne::Source;

use crate::parse::position::DocumentPosition;

#[derive(Debug, Clone, Copy)]
pub enum ParseErrTag {
    MissingExpression,
    UnmatchedParen,
    UnclosedTernary,
}

impl ParseErrTag {
    fn to_err_code(&self) -> usize {
        match self {
            ParseErrTag::MissingExpression => 4,
            ParseErrTag::UnmatchedParen => 5,
            ParseErrTag::UnclosedTernary => 6,
        }
    }
}

pub struct ParseErr {
    tag: ParseErrTag,
    filename: String,
    position: DocumentPosition,
}

impl ParseErr {
    pub fn new(tag: ParseErrTag, filename: String, position: DocumentPosition) -> Self {
        Self {
            tag,
            filename,
            position,
        }
    }

    pub fn print(&self, src: String) -> std::io::Result<()> {
        let report = self
            .position
            .to_report(self.filename.to_owned())
            .with_code(self.tag.to_err_code());

        match self.tag {
            ParseErrTag::MissingExpression => report
                .with_message("Missing expression")
                .with_note("Expected an expression, but didn't find one")
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("Expression expected here"),
                ),
            ParseErrTag::UnmatchedParen => report
                .with_message("Unmatched Parenthesis")
                .with_note("A paren is open, but never closed")
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("Paren opened here"),
                ),
            ParseErrTag::UnclosedTernary => report
                .with_message("Unfinished ternary expression")
                .with_note("A ternary statement is opened, but no corresponding ':' is found")
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("Ternary started here"),
                ),
        }
        .finish()
        .print((self.filename.to_owned(), Source::from(src)))
    }
}
