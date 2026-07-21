use crate::parse::{
    ast::expr::Expr,
    parse::err::{ParseErr, ParseErrTag},
    token::{Token, TokenTag},
};

pub struct Parser {
    filename: String,
    tokens: Vec<Token>,
    current: usize,
}

type ExprRes = Result<Expr, ParseErr>;

// utility methods
impl Parser {
    pub fn new(tokens: Vec<Token>, filename: String) -> Parser {
        Parser {
            filename,
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("This to never fail")
    }

    fn at_end(&self) -> bool {
        self.peek().tag == TokenTag::EOF
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("This to never fail")
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, tag: TokenTag) -> bool {
        if self.at_end() {
            return false;
        };
        self.peek().tag == tag
    }

    fn matches(&mut self, tags: Vec<TokenTag>) -> Option<&Token> {
        for tag in tags.iter() {
            if self.check(*tag) {
                return Some(self.advance());
            }
        }
        return None;
    }
}

// Main parser
impl Parser {
    pub fn parse(&mut self) -> ExprRes {
        return self.expression();
    }

    fn synchronise(&mut self) {
        self.advance();
        while !self.at_end() {
            if self.previous().tag == TokenTag::SemiColon {
                return;
            }

            match self.peek().tag {
                TokenTag::Class
                | TokenTag::Fun
                | TokenTag::Var
                | TokenTag::For
                | TokenTag::If
                | TokenTag::While
                | TokenTag::Print
                | TokenTag::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn expression(&mut self) -> ExprRes {
        self.sequence()
    }

    fn sequence(&mut self) -> ExprRes {
        // sequence -> ternary ("," ternary )* ;

        let mut expr = self.ternary()?;

        while let Some(op) = self.matches(vec![TokenTag::Comma]) {
            let op = op.try_into().expect("',' to be a bin op");
            let rhs = self.ternary()?;
            expr = Expr::binary(op, expr, rhs);
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> ExprRes {
        // ternary -> equality ("?" ternary ":" ternary)?
        let mut expr = self.equality()?;

        if let Some(token) = self.matches(vec![TokenTag::Question]) {
            let position = token.position.clone();
            let true_branch = self.ternary()?;

            if let None = self.matches(vec![TokenTag::Colon]) {
                return Err(ParseErr::new(
                    ParseErrTag::UnclosedTernary,
                    self.filename.clone(),
                    position,
                ));
            }

            let false_branch = self.ternary()?;

            expr = Expr::ternary(expr, true_branch, false_branch)
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ExprRes {
        // equality -> comparison ( ( "!=" | "==" ) comparison )* ;

        let mut expr = self.comparison()?;

        while let Some(op) = self.matches(vec![
            //
            TokenTag::BangEqual,
            TokenTag::EqualEqual,
        ]) {
            let op = op.try_into().expect("== and != to be bin ops");
            let rhs = self.comparison()?;
            expr = Expr::binary(op, expr, rhs);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ExprRes {
        // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let mut expr = self.term()?;

        while let Some(op) = self.matches(vec![
            TokenTag::Greater,
            TokenTag::GreaterEqual,
            TokenTag::Less,
            TokenTag::LessEqual,
        ]) {
            let op = op.try_into().expect(">, >=, <, <= to be bin ops");
            let rhs = self.term()?;
            expr = Expr::binary(op, expr, rhs)
        }

        Ok(expr)
    }

    fn term(&mut self) -> ExprRes {
        // term -> factor ( ( "+" | "-" ) factor )* ;
        let mut expr = self.factor()?;

        while let Some(op) = self.matches(vec![
            //
            TokenTag::Plus,
            TokenTag::Minus,
        ]) {
            let op = op.try_into().expect("+, - to be bin ops");
            let rhs = self.factor()?;
            expr = Expr::binary(op, expr, rhs)
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ExprRes {
        // factor -> unary ( ( "*" | "/" ) unary )* ;
        let mut expr = self.unary()?;

        while let Some(op) = self.matches(vec![
            //
            TokenTag::Slash,
            TokenTag::Star,
        ]) {
            let op = op.try_into().expect("/, * to be bin ops");
            let rhs = self.unary()?;
            expr = Expr::binary(op, expr, rhs)
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ExprRes {
        // unary -> ( "!" | "-" ) unary
        if let Some(op) = self.matches(vec![TokenTag::Bang, TokenTag::Minus]) {
            let op = op.try_into().expect("!, - to be unary ops");
            let arg = self.unary()?;
            return Ok(Expr::unary(op, arg));
        }

        //        | primary ;
        self.primary()
    }

    fn primary(&mut self) -> ExprRes {
        // primary -> NUMBER | STRING | "true" | "false" | "nil"
        //          | "(" expression ")" ;

        if let Some(_) = self.matches(vec![TokenTag::True]) {
            return Ok(Expr::tr());
        }

        if let Some(_) = self.matches(vec![TokenTag::False]) {
            return Ok(Expr::fls());
        }

        if let Some(_) = self.matches(vec![TokenTag::Nil]) {
            return Ok(Expr::Nil);
        }

        if let Some(token) = self.matches(vec![TokenTag::String]) {
            return Ok(Expr::StrLiteral {
                val: token.lexeme.clone(),
            });
        }

        if let Some(token) = self.matches(vec![TokenTag::Number]) {
            return Ok(Expr::NumLiteral {
                val: token
                    .lexeme
                    .parse()
                    .expect("num literals to parse correctly."),
            });
        }

        if let Some(opening_paren) = self.matches(vec![TokenTag::LeftParen]) {
            let position = opening_paren.position.clone(); // weird but necessary
            let val = self.expression()?;

            if let None = self.matches(vec![TokenTag::RightParen]) {
                return Err(ParseErr::new(
                    ParseErrTag::UnmatchedParen,
                    self.filename.clone(),
                    position,
                ));
            }

            return Ok(Expr::grouping(val));
        }

        return Err(ParseErr::new(
            ParseErrTag::MissingExpression,
            self.filename.clone(),
            self.peek().position.clone(),
        ));
    }
}
