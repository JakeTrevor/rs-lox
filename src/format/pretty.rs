use std::{
    fmt::Display,
    ops::{Add, BitOr},
};

#[derive(Clone, Debug)]
pub enum Doc {
    Nil,
    Seq(Box<Doc>, Box<Doc>),
    Nest { indent: usize, content: Box<Doc> },
    Text(String),
    Line,
    Or(Box<Doc>, Box<Doc>),
}

// Smart constructors
impl Doc {
    pub fn text(s: &str) -> Doc {
        Doc::Text(s.into())
    }

    fn seq(lhs: &Doc, rhs: &Doc) -> Doc {
        Doc::Seq(Box::new(lhs.clone()), Box::new(rhs.clone()))
    }

    pub fn nest(&self, indent: usize) -> Doc {
        Doc::Nest {
            indent,
            content: Box::new(self.clone()),
        }
    }

    pub fn or(lhs: &Doc, rhs: &Doc) -> Doc {
        Doc::Or(Box::new(lhs.clone()), Box::new(rhs.clone()))
    }

    pub fn prose(data: &str) -> Doc {
        data.trim()
            .split_whitespace()
            .map(|f| Doc::Text(f.into()))
            .fold(Doc::Nil, |acc, val| match acc {
                Doc::Nil => val,
                x => x + (Doc::text(" ") | Doc::Line) + val,
            })
    }

    pub fn break_space() -> Doc {
        Doc::Text(" ".into()) | Doc::Line
    }
}

// Impl Add(+) and BitOr(|) for nice syntax
impl Add for &Doc {
    type Output = Doc;

    fn add(self, rhs: Self) -> Self::Output {
        Doc::seq(self, rhs)
    }
}

impl Add for Doc {
    type Output = Doc;

    fn add(self, rhs: Self) -> Self::Output {
        Doc::seq(&self, &rhs)
    }
}

impl BitOr for &Doc {
    type Output = Doc;

    fn bitor(self, rhs: Self) -> Self::Output {
        Doc::or(self, rhs)
    }
}

impl BitOr for Doc {
    type Output = Doc;

    fn bitor(self, rhs: Self) -> Self::Output {
        Doc::or(&self, &rhs)
    }
}

impl Default for Doc {
    fn default() -> Self {
        Doc::Nil
    }
}

impl From<&str> for Doc {
    fn from(value: &str) -> Self {
        Doc::text(value)
    }
}

impl From<String> for Doc {
    fn from(value: String) -> Self {
        Doc::Text(value)
    }
}

pub fn fill(xs: &[Doc]) -> Doc {
    match xs {
        [] => Doc::Nil,
        [x] => x.clone(),
        [x, y, rest @ ..] => fill_from(x.clone(), y.clone(), rest),
    }
}

fn fill_from(x: Doc, y: Doc, rest: &[Doc]) -> Doc {
    let left = x.flatten() + Doc::text(" ") + fill_with_head(y.flatten(), rest);
    let right = x + Doc::Line + fill_with_head(y, rest);

    left | right
}

fn fill_with_head(head: Doc, rest: &[Doc]) -> Doc {
    match rest {
        [] => head,
        [x, xs @ ..] => fill_from(head, x.clone(), xs),
    }
}

pub trait Fillable: Iterator<Item = Doc> {
    fn join(self) -> Self::Item
    where
        Self: Sized,
    {
        self.fold(Doc::Nil, |acc, val| match acc {
            Doc::Nil => val,
            doc => doc + Doc::Line + val,
        })
    }
}

impl<'a, I: Iterator<Item = Doc>> Fillable for I {}

impl Doc {
    // Main algorithm:
    pub fn flatten(&self) -> Self {
        match self {
            Doc::Nil => Doc::Nil,
            Doc::Seq(a, b) => a.flatten() + b.flatten(),
            Doc::Nest { indent: _, content } => content.flatten(),
            Doc::Text(s) => Doc::text(s),
            Doc::Line => Doc::text(" "),
            // Invariant:
            // Forall (a b : Doc), (a | b).valid => a.flatten() = b.flatten()
            // Makes the following safe:
            Doc::Or(doc, _) => doc.flatten(),
            // As a result, Doc::Or should not be public
        }
    }

    pub fn group(&self) -> Self {
        self.flatten() | self.clone()
    }

    fn be(width: usize, current: usize, layouts: &mut Vec<(usize, Doc)>) -> NormDoc {
        match layouts.pop() {
            None => NormDoc::Nil,
            Some((_, Doc::Nil)) => Doc::be(width, current, layouts),
            Some((i, Doc::Seq(a, b))) => {
                layouts.push((i, *b));
                layouts.push((i, *a));
                Doc::be(width, current, layouts)
            }
            Some((i, Doc::Nest { indent, content })) => {
                layouts.push((i + indent, *content));
                Doc::be(width, current, layouts)
            }
            Some((_, Doc::Text(str))) => {
                let len = str.chars().count();
                NormDoc::text(str.to_owned(), Doc::be(width, current + len, layouts))
            }

            Some((i, Doc::Line)) => NormDoc::line(i, Doc::be(width, i, layouts)),
            Some((i, Doc::Or(a, b))) => {
                let mut rest = layouts.clone();
                rest.push((i, *a));
                let a = Doc::be(width, current, &mut rest);

                if width > current && a.fits(width - current) {
                    a
                } else {
                    layouts.push((i, *b));
                    Doc::be(width, current, layouts)
                }
            }
        }
    }

    fn best(self, width: usize) -> NormDoc {
        let mut v = vec![(0, self)];
        Doc::be(width, 0, &mut v)
    }

    // Utility Functions;
    pub fn bracket(self, l: &str, r: &str) -> Self {
        (Doc::text(l) + (Doc::Line + self).nest(2) + Doc::Line + Doc::text(r)).group()
    }
}

enum NormDoc {
    Nil,
    Text { content: String, rest: Box<NormDoc> },
    Line { indent: usize, rest: Box<NormDoc> },
}

impl NormDoc {
    fn text(content: String, rest: NormDoc) -> NormDoc {
        NormDoc::Text {
            content,
            rest: Box::new(rest),
        }
    }

    fn line(indent: usize, rest: NormDoc) -> NormDoc {
        NormDoc::Line {
            indent,
            rest: Box::new(rest),
        }
    }

    fn fits(&self, width: usize) -> bool {
        match self {
            NormDoc::Nil => true,
            NormDoc::Line { indent: _, rest: _ } => true,
            NormDoc::Text { content, rest } => {
                width >= content.len() && rest.fits(width - content.len())
            }
        }
    }
}

// This is 'Layout' from Wadler's Paper
impl Display for NormDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormDoc::Nil => write!(f, ""),
            NormDoc::Text { content, rest } => {
                content.fmt(f)?;
                rest.fmt(f)
            }
            NormDoc::Line { indent, rest } => {
                write!(f, "\n{:width$}{}", " ", rest, width = indent,)
            }
        }
    }
}

pub trait Pretty {
    fn to_doc(&self) -> Doc;

    // Anything that implements the pretty trait can be pretty printed
    fn pretty(&self, target_line_width: usize) -> String {
        self.to_doc().best(target_line_width).to_string()
    }

    fn pretty_print(&self, target_line_width: usize) {
        print!("{}", self.to_doc().best(target_line_width))
    }
}
