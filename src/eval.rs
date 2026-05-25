pub use crate::ast::{Expr, Stmt};
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
