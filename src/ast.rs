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
pub struct  LetStmt {
    pub name: String,
    pub expr: Expr
}

#[derive(Clone)]
pub struct  AssignStmt {
    pub name: String,
    pub expr: Expr
}

#[derive(Clone)]
pub struct  RetStmt {
    pub expr: Expr
}

#[derive(Clone)]
pub struct  IfStmt {
    pub branches: Vec<CondBlock>,
    pub else_body: Option<Block>,
}

#[derive(Clone)]
pub struct  WhileStmt {
    pub block: CondBlock
}

#[derive(Clone)]
pub struct  FnDeclStmt {
    pub name: String,
    pub args: Vec<String>,
    pub body: Block,
}

#[derive(Clone)]
pub struct  ExprStmt {
    pub expr: Expr,
}

#[derive(Clone)]
pub struct  ForInStmt {
    pub lhs: Expr,
    pub rhs: Expr,
    pub block: Block
}

#[derive(Clone)]
pub struct  ForICMStmt {
    pub init: Box<Stmt>,
    pub cond: Expr,
    pub fmod: Box<Stmt>,
    pub block: Block
}

#[derive(Clone)]
pub enum Stmt {
    Assign(AssignStmt),
    Let(LetStmt),
    Ret(RetStmt),
    If(IfStmt),
    While(WhileStmt),
    FnDecl(FnDeclStmt),
    Expr(ExprStmt),
    ForIn(ForInStmt),
    ForICM(ForICMStmt), // init; cond; mod
    Continue,
    Break
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
    use_eos: bool
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
        if !self.at(t.clone()) {
            Err(ParserError::UnexpectedToken{expected: t, got: self.peek()})
        } else {
            Ok(self.advance())
        }
    }

    pub fn enable_eos(&mut self, yes: bool) {
        self.use_eos = yes;
    }

    pub fn ensure_eos(&mut self) -> Result<Token, ParserError> {
        if self.use_eos {
            self.expect(Token::Eos)
        } else {
            Ok(Token::Eos)
        }
    }

    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0, use_eos: true }
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
