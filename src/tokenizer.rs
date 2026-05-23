use std::fmt::Display;

use crate::error::LexerError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Ret,
    If,
    Elif,
    Else,
    While,
    Let,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbrack,
    Rbrack,
    Logor,
    Logand,
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le,
    Add,
    Min,
    Mul,
    Div,
    Mod,
    Pwr,
    Not,
    Bnot,
    Bitor,
    Bitxor,
    Bitand,
    Shl,
    Shr,
    Comma,
    Assign,
    Eos,
    Eof,

    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum NumT {
    Norm,
    Hex,
    Oct,
    Bin,
}

struct Tokenizer {
    pos: usize,
    src: String,
    toks: Vec<Token>,
}

impl Tokenizer {
    fn at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn src_idx(&self, idx: usize) -> Option<char> {
        self.src.chars().nth(idx)
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src_idx(self.pos).unwrap()
        }
    }

    fn peek_n(&self, n: usize) -> char {
        if self.pos + n >= self.src.len() {
            '\0'
        } else {
            self.src_idx(self.pos + n).unwrap()
        }
    }

    fn advance(&mut self) -> char {
        if self.at_end() {
            '\0'
        } else {
            let c = self.src_idx(self.pos).unwrap();
            self.pos += 1;
            c
        }
    }

    fn advance_n(&mut self, n: usize) -> char {
        if self.at_end() {
            '\0'
        } else {
            let c = self.src_idx(self.pos + n).unwrap();
            self.pos += n;
            c
        }
    }

    fn skip_ws(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn push(&mut self, tk: Token) {
        self.toks.push(tk);
    }

    fn starts_with(&self, s: &str) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    fn parse_idname(&mut self) -> Result<String, LexerError> {
        self.skip_ws();
        if !self.peek().is_alphabetic() && self.peek() != '_' {
            return Err(LexerError::ExpectedIdentifier);
        }
        let mut name = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            name.push(self.advance());
        }
        Ok(name)
    }

    fn is_keyword(&self) -> bool {
        let kw = |s: &str| {
            if !self.starts_with(s) {
                return false;
            }
            let next = self.peek_n(s.len());
            !next.is_alphanumeric() && next != '_'
        };
        kw("fn")
            || kw("ret")
            || kw("elif")
            || kw("else")
            || kw("if")
            || kw("let")
            || kw("true")
            || kw("false")
            || kw("while")
    }

    fn is_ident(&self) -> bool {
        (self.peek().is_alphabetic() || self.peek() == '_') && !self.is_keyword()
    }

    fn new(src: String) -> Self {
        Self {
            pos: 0,
            src,
            toks: Vec::<Token>::new(),
        }
    }
}

pub fn tokenize(src: String) -> Result<Vec<Token>, LexerError> {
    let mut tk = Tokenizer::new(src);

    let mut imptd = Vec::<String>::new();
    while !tk.at_end() {
        tk.skip_ws();
        if tk.starts_with("import") {
            let stmt_start = tk.pos;
            tk.advance_n(6);
            tk.skip_ws();
            let mut imp_path = String::new();

            if tk.peek() == '"' {
                tk.advance();
                while tk.peek() != '"' {
                    imp_path.push(tk.advance());
                }
                tk.advance();
            } else if tk.is_ident() {
                imp_path = tk.parse_idname()? + ".f4na";
            } else {
                return Err(LexerError::ExpectedIdentifier);
            }

            if imptd.contains(&imp_path) {
                if tk.peek() != ';' {
                    return Err(LexerError::ExpectedToken(";".to_string()));
                }
                tk.advance();
                tk.src.drain(stmt_start..tk.pos);
                tk.pos = stmt_start;
                continue;
            }

            let mut import_src = std::fs::read_to_string(&imp_path)
                .map_err(|_| LexerError::FileNotFound(imp_path.clone()))?;

            if tk.peek() != ';' {
                return Err(LexerError::ExpectedToken(";".to_string()));
            }

            import_src.push('\n');

            tk.advance();
            tk.src
                .replace_range(stmt_start..tk.pos, import_src.as_str());
            imptd.push(imp_path);
            tk.pos = stmt_start;
        } else {
            tk.advance();
        }
    }

    tk.pos = 0;

    while !tk.at_end() {
        tk.skip_ws();
        let cr = tk.peek();

        if cr == '"' {
            tk.advance();
            let mut buf = String::new();
            while tk.peek() != '"' && !tk.at_end() {
                buf.push(tk.advance());
            }
            tk.advance();
            tk.push(Token::String(buf));
        } else if tk.is_keyword() {
            if tk.starts_with("fn") {
                tk.advance_n(2);
                tk.push(Token::Fn);
            } else if tk.starts_with("ret") {
                tk.advance_n(3);
                tk.push(Token::Ret);
            } else if tk.starts_with("if") {
                tk.advance_n(2);
                tk.push(Token::If);
            } else if tk.starts_with("elif") {
                tk.advance_n(4);
                tk.push(Token::Elif);
            } else if tk.starts_with("else") {
                tk.advance_n(4);
                tk.push(Token::Else);
            } else if tk.starts_with("let") {
                tk.advance_n(3);
                tk.push(Token::Let);
            } else if tk.starts_with("true") {
                tk.advance_n(4);
                tk.push(Token::Bool(true));
            } else if tk.starts_with("false") {
                tk.advance_n(5);
                tk.push(Token::Bool(false));
            } else if tk.starts_with("while") {
                tk.advance_n(5);
                tk.push(Token::While);
            }
        } else if cr == '(' {
            tk.advance();
            tk.push(Token::Lparen);
        } else if cr == ')' {
            tk.advance();
            tk.push(Token::Rparen);
        } else if cr == '{' {
            tk.advance();
            tk.push(Token::Lbrace);
        } else if cr == '}' {
            tk.advance();
            tk.push(Token::Rbrace);
        } else if cr == '[' {
            tk.advance();
            tk.push(Token::Lbrack);
        } else if cr == ']' {
            tk.advance();
            tk.push(Token::Rbrack);
        } else if tk.starts_with("||") {
            tk.advance_n(2);
            tk.push(Token::Logor);
        } else if tk.starts_with("&&") {
            tk.advance_n(2);
            tk.push(Token::Logand);
        } else if tk.starts_with("==") {
            tk.advance_n(2);
            tk.push(Token::Eq);
        } else if tk.starts_with("!=") {
            tk.advance_n(2);
            tk.push(Token::Neq);
        } else if cr == '|' {
            tk.advance();
            tk.push(Token::Bitor);
        } else if cr == '&' {
            tk.advance();
            tk.push(Token::Bitand);
        } else if cr == '^' {
            tk.advance();
            tk.push(Token::Bitxor);
        } else if cr == '>' {
            tk.advance();
            if tk.peek() == '=' {
                tk.advance();
                tk.push(Token::Ge);
            } else if tk.peek() == '>' {
                tk.advance();
                tk.push(Token::Shr);
            } else {
                tk.push(Token::Gt);
            }
        } else if cr == '<' {
            tk.advance();
            if tk.peek() == '=' {
                tk.advance();
                tk.push(Token::Le);
            } else if tk.peek() == '<' {
                tk.advance();
                tk.push(Token::Shl);
            } else {
                tk.push(Token::Lt);
            }
        } else if cr == '+' {
            tk.advance();
            tk.push(Token::Add);
        } else if cr == '-' {
            tk.advance();
            tk.push(Token::Min);
        } else if cr == '*' {
            tk.advance();
            if tk.peek() == '*' {
                tk.advance();
                tk.push(Token::Pwr);
            } else {
                tk.push(Token::Mul);
            }
        } else if cr == '/' {
            tk.advance();
            tk.push(Token::Div);
        } else if cr == '%' {
            tk.advance();
            tk.push(Token::Mod);
        } else if cr == '!' {
            tk.advance();
            tk.push(Token::Not);
        } else if cr == '~' {
            tk.advance();
            tk.push(Token::Bnot);
        } else if cr == ',' {
            tk.advance();
            tk.push(Token::Comma);
        } else if cr == '=' {
            tk.advance();
            tk.push(Token::Assign);
        } else if tk.is_ident() {
            let ident = tk.parse_idname()?;
            tk.push(Token::Ident(ident));
        } else if cr == ';' {
            tk.advance();
            tk.push(Token::Eos);
        } else {
            let mut buf = String::new();
            let mut nt = NumT::Norm;

            if tk.starts_with("0x") {
                nt = NumT::Hex;
                tk.advance_n(2);
            } else if tk.starts_with("0o") {
                nt = NumT::Oct;
                tk.advance_n(2);
            } else if tk.starts_with("0b") {
                nt = NumT::Bin;
                tk.advance_n(2);
            }

            let isok = |c: char| -> bool {
                (nt == NumT::Norm && c.is_ascii_digit())
                    || (nt == NumT::Hex && c.is_ascii_hexdigit())
                    || (nt == NumT::Oct && ('0'..='7').contains(&c))
                    || (nt == NumT::Bin && (c == '0' || c == '1'))
            };

            while isok(tk.peek()) {
                buf.push(tk.advance());
            }

            if nt == NumT::Norm && tk.peek() == '.' {
                buf.push(tk.advance());
                while tk.peek().is_ascii_digit() {
                    buf.push(tk.advance());
                }
                tk.push(Token::Float(
                    buf.parse::<f64>().map_err(|_| LexerError::BadNumber)?,
                ))
            } else {
                let base = match nt {
                    NumT::Hex => 16,
                    NumT::Oct => 8,
                    NumT::Bin => 2,
                    NumT::Norm => 10,
                };

                let n = i64::from_str_radix(&buf, base).map_err(|_| LexerError::BadNumber)?;
                tk.push(Token::Int(n));
            }
        }
    }

    tk.push(Token::Eof);

    Ok(tk.toks)
}
