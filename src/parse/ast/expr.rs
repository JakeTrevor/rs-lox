use std::fmt::Display;

use crate::parse::token::{Token, TokenTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Plus,
    Minus,
    Times,
    Divide,
    Seq,
}

impl TryFrom<&TokenTag> for BinOp {
    type Error = ();

    fn try_from(value: &TokenTag) -> Result<Self, Self::Error> {
        match value {
            TokenTag::EqualEqual => Ok(BinOp::Eq),
            TokenTag::BangEqual => Ok(BinOp::Neq),
            TokenTag::Less => Ok(BinOp::Lt),
            TokenTag::LessEqual => Ok(BinOp::Lte),
            TokenTag::Greater => Ok(BinOp::Gt),
            TokenTag::GreaterEqual => Ok(BinOp::Gte),
            TokenTag::Minus => Ok(BinOp::Minus),
            TokenTag::Plus => Ok(BinOp::Plus),
            TokenTag::Slash => Ok(BinOp::Divide),
            TokenTag::Star => Ok(BinOp::Times),
            TokenTag::Comma => Ok(BinOp::Seq),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Token> for BinOp {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        (&value.tag).try_into()
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            BinOp::Eq => "==",
            BinOp::Neq => "!=",
            BinOp::Lt => "<",
            BinOp::Lte => "<=",
            BinOp::Gt => ">",
            BinOp::Gte => ">=",
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Times => "*",
            BinOp::Divide => "/",
            BinOp::Seq => ",",
        };

        write!(f, " {op} ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
}

impl TryFrom<&TokenTag> for UnOp {
    type Error = ();

    fn try_from(value: &TokenTag) -> Result<Self, Self::Error> {
        match value {
            TokenTag::Bang => Ok(UnOp::Not),
            TokenTag::Minus => Ok(UnOp::Neg),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Token> for UnOp {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        (&value.tag).try_into()
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            UnOp::Not => "!",
            UnOp::Neg => "-",
        };

        write!(f, "{op}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    NumLiteral {
        val: f64,
    },
    StrLiteral {
        val: String,
    },
    BoolLiteral {
        val: bool,
    },
    Nil,
    Unary {
        op: UnOp,
        arg: Box<Expr>,
    },
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Grouping {
        val: Box<Expr>,
    },
}

impl Expr {
    pub fn tr() -> Expr {
        Expr::BoolLiteral { val: true }
    }
    pub fn fls() -> Expr {
        Expr::BoolLiteral { val: false }
    }

    pub fn unary(op: UnOp, arg: Expr) -> Expr {
        Expr::Unary {
            op,
            arg: Box::new(arg),
        }
    }

    pub fn binary(op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn grouping(val: Expr) -> Expr {
        Expr::Grouping { val: Box::new(val) }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::NumLiteral { val } => write!(f, "{val}"),
            Expr::StrLiteral { val } => write!(f, "{val}"),
            Expr::BoolLiteral { val } => write!(f, "{val}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Unary { op, arg } => write!(f, "{op}{arg}"),
            Expr::Binary { op, lhs, rhs } => write!(f, "{lhs}{op}{rhs}"),
            Expr::Grouping { val } => write!(f, "({val})"),
        }
    }
}
