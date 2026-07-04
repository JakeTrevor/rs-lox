use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
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
        };

        write!(f, "{op}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnOp {
    Not,
    Neg,
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
enum Expr {
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::NumLiteral { val } => write!(f, "{val}"),
            Expr::StrLiteral { val } => write!(f, "{val}"),
            Expr::BoolLiteral { val } => write!(f, "{val}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Unary { op, arg } => write!(f, "{op}{arg}"),
            Expr::Binary { op, lhs, rhs } => write!(f, "{lhs} {op} {rhs}"),
            Expr::Grouping { val } => write!(f, "({val})"),
        }
    }
}
