pub use crate::error::ParserError;
pub use crate::tokenizer::Token;
pub use crate::value::Value;

#[derive(Clone)]
pub enum Expr {
    Binary {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        arg: Box<Expr>,
    },
    Call {
        ident: String,
        args: Vec<Expr>,
    },
    Index {
        expr: Box<Expr>,
        idx: Box<Expr>,
    },
    Ident {
        ident: String,
    },
    Literal {
        val: Value,
    },
    ArrayLiteral {
        members: Vec<Expr>,
    },
}

pub type Block = Vec<Stmt>;

#[derive(Clone)]
pub struct CondBlock {
    pub cond: Expr,
    pub body: Block,
}

#[derive(Clone)]
pub enum Stmt {
    Let {
        name: String,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    Ret {
        expr: Expr,
    },
    If {
        branches: Vec<CondBlock>,
        else_body: Option<Block>,
    },
    While {
        block: CondBlock,
    },
    FnDecl {
        name: String,
        args: Vec<String>,
        body: Block,
    },
    Expr {
        expr: Expr,
    },
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    pub fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.tokens[self.pos] == Token::Eof
    }

    pub fn peek(&self) -> Token {
        if self.at_end() {
            return Token::Eof;
        }
        self.tokens[self.pos].clone()
    }

    pub fn peek_n(&self, n: usize) -> Token {
        if self.pos + n >= self.tokens.len() {
            return Token::Eof;
        }
        self.tokens[self.pos + n].clone()
    }

    pub fn advance(&mut self) -> Token {
        if self.at_end() {
            return Token::Eof;
        }
        let tk = self.tokens[self.pos].clone();
        self.pos += 1;
        tk
    }

    pub fn advance_n(&mut self, n: usize) -> Token {
        if self.pos + n >= self.tokens.len() {
            return Token::Eof;
        }
        let tk = self.tokens[self.pos].clone();
        self.pos += n;
        tk
    }

    pub fn at(&self, t: Token) -> bool {
        self.peek() == t
    }

    pub fn expect(&mut self, t: Token) -> Result<Token, ParserError> {
        if !self.at(t) {
            Err(ParserError::UnexpectedToken)
        } else {
            Ok(self.advance())
        }
    }

    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
}

pub fn parse(toks: Vec<Token>) -> Result<Vec<Stmt>, ParserError> {
    let mut psr = Parser::new(toks);
    let mut statements = Vec::<Stmt>::new();
    while !psr.at_end() {
        statements.push(crate::ast_stmt::parse_stmt(&mut psr)?);
    }
    Ok(statements)
}
