pub use crate::ast::{Expr, Stmt};
use crate::eval_stmt::eval_stmt;
pub use crate::error::RuntimeError;
use crate::{ast::Block, value::Value};
use std::collections::HashMap;

pub type BuiltinLUT = HashMap<String, BuiltinFn>;
pub type UserFnLUT = HashMap<String, UserFn>;
pub type VarLUT = HashMap<String, Value>;

pub struct BuiltinFn {
    pub nargs: isize,
    pub fnc: fn(Vec<Value>) -> Value,
}

pub struct UserFn {
    pub args: Vec<String>,
    pub body: Block,
    pub vars: VarLUT,
}

#[derive(Default)]
pub struct Environment {
    pub vars: VarLUT,
    pub bfnlut: BuiltinLUT,
    pub ufnlut: UserFnLUT,
}

pub type ResValue = Result<Value, RuntimeError>;
pub type ResVoid = Result<(), RuntimeError>;


pub fn eval(prog: Vec<Stmt>) -> ResVoid {
    let mut env = crate::build_env::build_env().unwrap();
    for stmt in prog {
        eval_stmt(&mut env, &stmt)?;
    }
    Ok(())
}