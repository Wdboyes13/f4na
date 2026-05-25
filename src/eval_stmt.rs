use crate::ast::*;
use crate::eval::*;
use crate::eval_expr::eval_expr;

fn eval_let(env: &mut Environment, stmt: &LetStmt) -> ResVoid {
    let val = eval_expr(env, &stmt.expr)?;
    env.vars.insert(stmt.name.clone(), val);
    Ok(())
}

fn eval_assign(env: &mut Environment, stmt: &AssignStmt) -> ResVoid {
    let val = eval_expr(env, &stmt.expr)?;
    if env.vars.contains_key(&stmt.name) {
        match env.vars.get_mut(&stmt.name) {
            Some(v) => *v = val,
            None => panic!("shouldn't reach this line"),
        };

        Ok(())
    } else {
        Err(RuntimeError::UnknownIdent(stmt.name.clone()))
    }
}

fn eval_ret(env: &mut Environment, stmt: &RetStmt) -> ResVoid {
    Err(RuntimeError::Return(eval_expr(env, &stmt.expr)?))
}

fn eval_if(env: &mut Environment, stmt: &IfStmt) -> ResVoid {
    for branch in &stmt.branches {
        if eval_expr(env, &branch.cond)?.as_bool() {
            return eval_block(env, &branch.body);
        }
    }

    if let Some(else_body) = &stmt.else_body {
        eval_block(env, else_body)
    } else {
        Ok(())
    }
}

fn eval_fndecl(env: &mut Environment, stmt: &FnDeclStmt) -> ResVoid {
    env.ufnlut.insert(
        stmt.name.clone(),
        UserFn {
            args: stmt.args.clone(),
            body: stmt.body.clone(),
            vars: env.vars.clone(),
        },
    );
    Ok(())
}

fn eval_exprstmt(env: &mut Environment, stmt: &ExprStmt) -> ResVoid {
    eval_expr(env, &stmt.expr)?;
    Ok(())
}

fn eval_while(env: &mut Environment, stmt: &WhileStmt) -> ResVoid {
    while eval_expr(env, &stmt.block.cond)?.as_bool() {
            eval_block(env, &stmt.block.body)?;
    }
    Ok(())

}

fn eval_forin(env: &mut Environment, stmt: &ForInStmt) -> ResVoid {
    let rhs = eval_expr(env, &stmt.rhs)?; 
    if let Expr::Ident{ident} = stmt.lhs.clone() {
        let mut old_val: Option<Value> = None;
        if env.vars.contains_key(&ident) {
            old_val = Some(env.vars[&ident].clone());
        }
        match rhs {
            Value::Int(i) => {
                for x in 0i64..i {
                    env.vars.insert(ident.clone(), Value::Int(x));
                    eval_block(env, &stmt.block)?;
                }
            },
            Value::Array(a) => {
                for x in a {
                    env.vars.insert(ident.clone(), x);
                    eval_block(env, &stmt.block)?;
                }
            },
            Value::Bool(_) => {
                return Err(RuntimeError::TypeError("cannot use bool for 'for in'".to_string()));
            },
            Value::String(s) => {
                for c in s.chars() {
                    env.vars.insert(ident.clone(), Value::String(c.to_string()));
                    eval_block(env, &stmt.block)?;
                }
            },
            Value::Float(f) => {
                for x in 0i64..(f as i64) {
                    env.vars.insert(ident.clone(), Value::Float(x as f64));
                    eval_block(env, &stmt.block)?;
                }
            }
        }
        if let Some(val) = old_val {
            env.vars.insert(ident, val);
        } else {
            env.vars.remove(&ident);
        }
        Ok(())
    } else {
        Err(RuntimeError::TypeError("expected identifier for 'for in'".to_string()))
    }
}

fn eval_foricm(env: &mut Environment, stmt: &ForICMStmt) -> ResVoid {
    eval_stmt(env, &stmt.init)?;

    while eval_expr(env, &stmt.cond)?.as_bool() {
        eval_block(env, &stmt.block)?;
        eval_stmt(env, &stmt.fmod)?;
    }

    Ok(())
}

pub fn eval_stmt(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    match stmt {
        Stmt::Let(stmt) => eval_let(env, stmt),
        Stmt::Assign(stmt) => eval_assign(env, stmt),
        Stmt::Ret(stmt) => eval_ret(env, stmt),
        Stmt::If(stmt) => eval_if(env, stmt),
        Stmt::While(stmt) => eval_while(env, stmt),
        Stmt::FnDecl(stmt) => eval_fndecl(env, stmt),
        Stmt::Expr(stmt) => eval_exprstmt(env, stmt),
        Stmt::ForIn(stmt) => eval_forin(env, stmt),
        Stmt::ForICM(stmt) => eval_foricm(env, stmt)
    }
}

pub fn eval_block(env: &mut Environment, blk: &Block) -> ResVoid {
    for stmt in blk {
        eval_stmt(env, stmt)?;
    }

    Ok(())
}