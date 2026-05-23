use crate::ast::Block;
use crate::ast::Stmt;
use crate::build_env;
use crate::eval::*;
use crate::eval_expr::eval_expr;

fn eval_let(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::Let { name, expr } = stmt {
        let val = eval_expr(env, expr)?;
        env.vars.insert(name.clone(), val);
        Ok(())
    } else {
        panic!("bad statement");
    }
}

fn eval_assign(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::Assign { name, expr } = stmt {
        let val = eval_expr(env, expr)?;
        if env.vars.contains_key(name) {
            match env.vars.get_mut(name) {
                Some(v) => *v = val,
                None => panic!("shouldn't reach this line"),
            };

            Ok(())
        } else {
            Err(RuntimeError::UnknownIdent(name.clone()))
        }
    } else {
        panic!("bad statement");
    }
}

fn eval_ret(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::Ret { expr } = stmt {
        Err(RuntimeError::Return(eval_expr(env, expr)?))
    } else {
        panic!("bad statement");
    }
}

fn eval_if(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::If {
        branches,
        else_body,
    } = stmt
    {
        for branch in branches {
            if eval_expr(env, &branch.cond)?.as_bool() {
                return eval_block(env, &branch.body);
            }
        }

        if else_body.is_some() {
            eval_block(env, else_body.as_ref().unwrap())
        } else {
            Ok(())
        }
    } else {
        panic!("bad statement");
    }
}

fn eval_fndecl(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::FnDecl { name, args, body } = stmt {
        env.ufnlut.insert(
            name.clone(),
            UserFn {
                args: args.clone(),
                body: body.clone(),
                vars: env.vars.clone(),
            },
        );
        Ok(())
    } else {
        panic!("bad statement");
    }
}

fn eval_exprstmt(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::Expr { expr } = stmt {
        eval_expr(env, expr)?;
        Ok(())
    } else {
        panic!("bad statement");
    }
}

fn eval_while(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    if let Stmt::While { block } = stmt {
        while eval_expr(env, &block.cond)?.as_bool() {
            eval_block(env, &block.body)?;
        }
        Ok(())
    } else {
        panic!("bad statement");
    }
}

fn eval_stmt(env: &mut Environment, stmt: &Stmt) -> ResVoid {
    match stmt {
        Stmt::Let { name: _, expr: _ } => eval_let(env, stmt),
        Stmt::Assign { name: _, expr: _ } => eval_assign(env, stmt),
        Stmt::Ret { expr: _ } => eval_ret(env, stmt),
        Stmt::If {
            branches: _,
            else_body: _,
        } => eval_if(env, stmt),
        Stmt::While { block: _ } => eval_while(env, stmt),
        Stmt::FnDecl {
            name: _,
            args: _,
            body: _,
        } => eval_fndecl(env, stmt),
        Stmt::Expr { expr: _ } => eval_exprstmt(env, stmt),
    }
}

pub fn eval_block(env: &mut Environment, blk: &Block) -> ResVoid {
    for stmt in blk {
        eval_stmt(env, stmt)?;
    }

    Ok(())
}

pub fn eval(prog: Vec<Stmt>) -> ResVoid {
    let mut env = build_env::build_env().unwrap();
    for stmt in prog {
        eval_stmt(&mut env, &stmt)?;
    }
    Ok(())
}
