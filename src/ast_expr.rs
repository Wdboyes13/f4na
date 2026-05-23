use crate::ast::*;

pub type ResultExpr = Result<Expr, ParserError>;

pub fn parse_expr(p: &mut Parser) -> ResultExpr {
    parse_logor(p)
}

fn parse_logor(p: &mut Parser) -> ResultExpr {
    let mut left = parse_logand(p)?;
    while p.at(Token::Logor) {
        let op = p.advance();
        let right = parse_logand(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_logand(p: &mut Parser) -> ResultExpr {
    let mut left = parse_bitor(p)?;
    while p.at(Token::Logand) {
        let op = p.advance();
        let right = parse_bitor(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_bitor(p: &mut Parser) -> ResultExpr {
    let mut left = parse_bitxor(p)?;
    while p.at(Token::Bitor) {
        let op = p.advance();
        let right = parse_bitxor(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_bitxor(p: &mut Parser) -> ResultExpr {
    let mut left = parse_bitand(p)?;
    while p.at(Token::Bitand) {
        let op = p.advance();
        let right = parse_bitand(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_bitand(p: &mut Parser) -> ResultExpr {
    let mut left = parse_equ(p)?;
    while p.at(Token::Bitand) {
        let op = p.advance();
        let right = parse_equ(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_equ(p: &mut Parser) -> ResultExpr {
    let mut left = parse_comp(p)?;
    while p.at(Token::Eq) || p.at(Token::Neq) {
        let op = p.advance();
        let right = parse_comp(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_comp(p: &mut Parser) -> ResultExpr {
    let mut left = parse_shift(p)?;
    while p.at(Token::Gt) || p.at(Token::Lt) || p.at(Token::Ge) || p.at(Token::Le) {
        let op = p.advance();
        let right = parse_shift(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_shift(p: &mut Parser) -> ResultExpr {
    let mut left = parse_add(p)?;
    while p.at(Token::Shl) || p.at(Token::Shr) {
        let op = p.advance();
        let right = parse_add(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_add(p: &mut Parser) -> ResultExpr {
    let mut left = parse_term(p)?;
    while p.at(Token::Min) || p.at(Token::Add) {
        let op = p.advance();
        let right = parse_term(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_term(p: &mut Parser) -> ResultExpr {
    let mut left = parse_unary(p)?;
    while p.at(Token::Mul) || p.at(Token::Div) || p.at(Token::Mod) {
        let op = p.advance();
        let right = parse_unary(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_unary(p: &mut Parser) -> ResultExpr {
    if p.at(Token::Not) || p.at(Token::Add) || p.at(Token::Min) || p.at(Token::Bnot) {
        let op = p.advance();
        return Ok(Expr::Unary {
            op,
            arg: Box::new(parse_unary(p)?),
        });
    }

    parse_power(p)
}

fn parse_power(p: &mut Parser) -> ResultExpr {
    let mut left = parse_primary(p)?;
    if p.at(Token::Pwr) {
        let op = p.advance();
        let right = parse_primary(p)?;

        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_primary(p: &mut Parser) -> ResultExpr {
    if p.at(Token::Lparen) {
        p.advance();
        let expr = parse_expr(p);

        p.expect(Token::Rparen)?;
        return expr;
    }

    let mut base: Expr;

    if let Token::Ident(id) = p.peek() {
        p.advance();
        if p.at(Token::Lparen) {
            p.advance();
            let mut args = Vec::<Expr>::new();
            while !p.at(Token::Rparen) {
                args.push(parse_expr(p)?);
                if p.at(Token::Comma) {
                    p.advance();
                }
            }

            p.advance();
            base = Expr::Call { ident: id, args };
        } else {
            base = Expr::Ident { ident: id };
        }
    } else {
        base = parse_literal(p)?;
    }

    while p.at(Token::Lbrack) {
        p.advance();
        let idx = parse_expr(p)?;
        p.expect(Token::Rbrack)?;
        base = Expr::Index {
            expr: Box::new(base),
            idx: Box::new(idx),
        };
    }

    Ok(base)
}

fn parse_literal(p: &mut Parser) -> ResultExpr {
    if p.at(Token::Lbrace) {
        return parse_aliteral(p);
    }

    if let Token::Int(i) = p.peek() {
        p.advance();
        Ok(Expr::Literal { val: Value::Int(i) })
    } else if let Token::Float(f) = p.peek() {
        p.advance();
        Ok(Expr::Literal {
            val: Value::Float(f),
        })
    } else if let Token::String(s) = p.peek() {
        p.advance();
        Ok(Expr::Literal {
            val: Value::String(s),
        })
    } else if let Token::Bool(b) = p.peek() {
        p.advance();
        Ok(Expr::Literal {
            val: Value::Bool(b),
        })
    } else {
        Err(ParserError::UnexpectedToken)
    }
}

fn parse_aliteral(p: &mut Parser) -> ResultExpr {
    p.expect(Token::Lbrace)?;
    let mut mbrs = Vec::<Expr>::new();

    while !p.at(Token::Rbrace) {
        mbrs.push(parse_expr(p)?);
        if p.at(Token::Comma) {
            p.advance();
        }
    }

    p.advance();

    Ok(Expr::ArrayLiteral { members: mbrs })
}
