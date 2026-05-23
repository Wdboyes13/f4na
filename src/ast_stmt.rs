use crate::ast::*;
use crate::ast_expr::parse_expr;

fn parse_block(p: &mut Parser) -> Result<Block, ParserError> {
    if !p.at(Token::Lbrace) {
        return Err(ParserError::ExpectedToken(Token::Lbrace));
    }
    p.advance();

    let mut body = Vec::<Stmt>::new();
    while !p.at(Token::Rbrace) {
        body.push(parse_stmt(p)?);
    }

    p.advance();
    Ok(body)
}

fn parse_condblk(p: &mut Parser) -> Result<CondBlock, ParserError> {
    let cond = parse_expr(p)?;
    let body = parse_block(p)?;
    Ok(CondBlock { cond, body })
}

fn parse_if(p: &mut Parser) -> Result<Stmt, ParserError> {
    let mut branches = Vec::<CondBlock>::new();
    let mut else_body: Option<Block> = None;

    branches.push(parse_condblk(p)?);

    while p.at(Token::Elif) {
        p.advance();
        branches.push(parse_condblk(p)?);
    }

    if p.at(Token::Else) {
        p.advance();
        else_body = Some(parse_block(p)?);
    }

    Ok(Stmt::If {
        branches,
        else_body,
    })
}

fn parse_fn(p: &mut Parser) -> Result<Stmt, ParserError> {
    if let Token::Ident(id) = p.advance() {
        if !p.at(Token::Lparen) {
            return Err(ParserError::ExpectedToken(Token::Lparen));
        }
        p.advance();

        let mut args = Vec::<String>::new();
        while !p.at(Token::Rparen) {
            if let Token::Ident(id) = p.peek() {
                args.push(id);
                p.advance();
            } else {
                return Err(ParserError::ExpectedIdent);
            }

            if p.at(Token::Comma) {
                p.advance();
            }
        }

        p.advance();

        let body = parse_block(p)?;
        Ok(Stmt::FnDecl {
            name: id,
            args,
            body,
        })
    } else {
        Err(ParserError::ExpectedIdent)
    }
}

fn parse_while(p: &mut Parser) -> Result<Stmt, ParserError> {
    Ok(Stmt::While {
        block: parse_condblk(p)?,
    })
}

pub fn parse_stmt(p: &mut Parser) -> Result<Stmt, ParserError> {
    if p.at(Token::Let) {
        p.advance();
        if let Token::Ident(id) = p.advance() {
            if p.at(Token::Assign) {
                p.advance();
            } else {
                return Err(ParserError::ExpectedToken(Token::Assign));
            }

            let expr = parse_expr(p)?;

            p.expect(Token::Eos)?;

            Ok(Stmt::Let { name: id, expr })
        } else {
            Err(ParserError::ExpectedIdent)
        }
    } else if let Token::Ident(id) = p.peek()
        && p.peek_n(1) == Token::Assign
    {
        p.advance_n(2);

        let expr = parse_expr(p)?;
        p.expect(Token::Eos)?;

        Ok(Stmt::Assign { name: id, expr })
    } else if p.at(Token::Ret) {
        p.advance();
        let expr = parse_expr(p)?;
        p.expect(Token::Eos)?;
        Ok(Stmt::Ret { expr })
    } else if p.at(Token::If) {
        p.advance();
        parse_if(p)
    } else if p.at(Token::Fn) {
        p.advance();
        parse_fn(p)
    } else if p.at(Token::While) {
        p.advance();
        parse_while(p)
    } else {
        let expr = parse_expr(p)?;
        p.expect(Token::Eos)?;
        Ok(Stmt::Expr { expr })
    }
}
