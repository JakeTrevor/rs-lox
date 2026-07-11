use ariadne::{Label, Report, ReportBuilder, ReportKind};

#[derive(Clone, Debug)]
pub struct DocumentPosition {
    pub(in crate::parse) offset: usize, // from start of file
    pub(in crate::parse) column: usize, // from start of line
    pub(in crate::parse) line: usize,
}

impl Default for DocumentPosition {
    fn default() -> Self {
        Self {
            offset: 0,
            column: 1,
            line: 1,
        }
    }
}

type Span = (String, std::ops::Range<usize>);

impl DocumentPosition {
    pub(in crate::parse) fn advance(&mut self) {
        self.offset += 1;
        self.column += 1;
    }

    pub(in crate::parse) fn newline(&mut self) {
        self.column = 1;
        self.line += 1;
    }

    pub(in crate::parse) fn set(&mut self, other: &DocumentPosition) {
        self.offset = other.offset;
        self.column = other.column;
        self.line = other.line;
    }

    pub fn to_report(&self, file: String) -> ReportBuilder<'_, Span> {
        return Report::build(ReportKind::Error, (file, (self.offset..self.offset)));
    }

    pub fn to_label(&self, file: String) -> Label<Span> {
        return Label::new((file, (self.offset..self.offset)));
    }
}
