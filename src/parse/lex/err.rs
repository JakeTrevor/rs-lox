use std::fmt::Debug;

use ariadne::Source;

use crate::parse::position::DocumentPosition;

#[derive(Debug)]
pub enum LexErrTag {
    UnexpectedCharacter,
    UnterminatedString,
    UnterminatedComment,
}

impl LexErrTag {
    fn to_err_code(&self) -> usize {
        match self {
            LexErrTag::UnexpectedCharacter => 1,
            LexErrTag::UnterminatedString => 2,
            LexErrTag::UnterminatedComment => 3,
        }
    }
}

pub struct LexErr {
    tag: LexErrTag,
    filename: String,
    position: DocumentPosition,
}

impl LexErr {
    pub fn new(tag: LexErrTag, filename: String, position: DocumentPosition) -> Self {
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
            LexErrTag::UnexpectedCharacter => report
                .with_message("Unexpected character")
                .with_note("This doesn't seem like valid syntax...")
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("This character"),
                ),
            LexErrTag::UnterminatedString => report
                .with_message("Unterminated string")
                .with_note(format!(
                    "A string is started at {}:{} but never closed",
                    self.position.line, self.position.column
                ))
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("String starts here"),
                ),
            LexErrTag::UnterminatedComment => report
                .with_message("Unterminated comment")
                .with_note(format!(
                    "A comment is started at {}:{} but never closed",
                    self.position.line, self.position.column
                ))
                .with_label(
                    self.position
                        .to_label(self.filename.to_owned())
                        .with_message("Comment is starts here"),
                ),
        }
        .finish()
        .print((self.filename.to_owned(), Source::from(src)))
    }
}
