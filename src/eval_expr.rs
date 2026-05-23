use crate::ast::Expr;
use crate::eval::*;
use crate::eval_stmt::eval_block;
use crate::tokenizer::Token;
use crate::value::Value;

fn eval_binary(env: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::Binary { op, left, right } = e {
        let lv = eval_expr(env, left.as_ref())?;
        let rv = eval_expr(env, right.as_ref())?;

        match op {
            Token::Logor => lv.logic_or(rv),
            Token::Logand => lv.logic_and(rv),
            Token::Eq => Ok(Value::Bool(lv == rv)),
            Token::Neq => Ok(Value::Bool(lv != rv)),
            Token::Gt => lv.val_gt(rv),
            Token::Lt => lv.val_lt(rv),
            Token::Ge => lv.val_ge(rv),
            Token::Le => lv.val_le(rv),
            Token::Min => lv - rv,
            Token::Add => lv + rv,
            Token::Mul => lv * rv,
            Token::Div => lv / rv,
            Token::Mod => lv % rv,
            Token::Pwr => lv.pow(rv),
            Token::Bitor => lv | rv,
            Token::Bitand => lv & rv,
            Token::Bitxor => lv ^ rv,
            Token::Shl => lv << rv,
            Token::Shr => lv >> rv,
            _ => panic!("bad expression"),
        }
    } else {
        panic!("bad expression");
    }
}

fn eval_unary(env: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::Unary { op, arg } = e {
        let val = eval_expr(env, arg)?;

        match op {
            Token::Not => !val,
            Token::Add => -val,
            Token::Bnot => val.bitwise_not(),
            _ => panic!("bad expression"),
        }
    } else {
        panic!("bad expression");
    }
}

fn eval_call(env: &mut Environment, e: &Expr) -> ResValue {
    let mut args = Vec::<Value>::new();
    if let Expr::Call { ident, args: eargs } = e {
        for arg in eargs {
            args.push(eval_expr(env, arg)?);
        }

        if env.bfnlut.contains_key(ident) {
            let fnc = &env.bfnlut[ident];
            if fnc.nargs as usize == args.len() || fnc.nargs == -1 {
                Ok((fnc.fnc)(args))
            } else {
                Err(RuntimeError::WrongArgCount {
                    expected: fnc.nargs as usize,
                    got: args.len(),
                })
            }
        } else if env.ufnlut.contains_key(ident) {
            let fnc = &env.ufnlut[ident];
            if fnc.args.len() != args.len() {
                return Err(RuntimeError::WrongArgCount {
                    expected: fnc.args.len(),
                    got: args.len(),
                });
            }

            let svars = env.vars.clone();
            env.vars = fnc.vars.clone();

            for i in 0..args.len() {
                env.vars.insert(fnc.args[i].to_string(), args[i].clone());
            }

            if let Err(RuntimeError::Return(v)) = eval_block(env, &fnc.body.clone()) {
                env.vars = svars;
                Ok(v)
            } else {
                Ok(Value::Int(0))
            }
        } else {
            Err(RuntimeError::UnknownFunction(ident.to_string()))
        }
    } else {
        panic!("bad expression");
    }
}

fn eval_ident(env: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::Ident { ident } = e {
        if env.vars.contains_key(ident) {
            Ok(env.vars[ident].clone())
        } else {
            Err(RuntimeError::UnknownIdent(ident.to_string()))
        }
    } else {
        panic!("bad expression");
    }
}

fn eval_literal(_: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::Literal { val } = e {
        Ok(val.clone())
    } else {
        panic!("bad expression");
    }
}

fn eval_aliteral(env: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::ArrayLiteral { members } = e {
        let mut vals = Vec::<Value>::new();
        for mb in members {
            vals.push(eval_expr(env, mb)?);
        }
        Ok(Value::Array(vals))
    } else {
        panic!("bad expression");
    }
}

fn eval_index(env: &mut Environment, e: &Expr) -> ResValue {
    if let Expr::Index { expr, idx } = e {
        if let Ok(Value::Array(a)) = eval_expr(env, expr) {
            let idx = eval_expr(env, idx)?;
            if let Value::Int(idx) = idx {
                Ok(a[idx as usize].clone())
            } else {
                panic!("cannot index array with non-int type");
            }
        } else {
            Err(RuntimeError::TypeError(
                "cannot index non-array type".to_string(),
            ))
        }
    } else {
        panic!("bad expression");
    }
}

pub fn eval_expr(env: &mut Environment, e: &Expr) -> ResValue {
    match *e {
        Expr::Binary {
            op: _,
            left: _,
            right: _,
        } => eval_binary(env, e),
        Expr::Unary { op: _, arg: _ } => eval_unary(env, e),
        Expr::Call { ident: _, args: _ } => eval_call(env, e),
        Expr::Index { expr: _, idx: _ } => eval_index(env, e),
        Expr::Ident { ident: _ } => eval_ident(env, e),
        Expr::Literal { val: _ } => eval_literal(env, e),
        Expr::ArrayLiteral { members: _ } => eval_aliteral(env, e),
    }
}
