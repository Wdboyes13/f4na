use crate::error::RuntimeError;
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl Add for Value {
    type Output = Result<Value, RuntimeError>;
    fn add(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
            (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l.clone() + r)),
            (Value::Array(l), _) => {
                let mut arr = l.clone();
                arr.push(b);
                Ok(Value::Array(arr))
            }
            _ => Err(RuntimeError::TypeError("Unknown type".to_string())),
        }
    }
}

use std::ops::{BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

impl Sub for Value {
    type Output = Result<Value, RuntimeError>;
    fn sub(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
            (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 - r)),
            (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l - *r as f64)),
            _ => Err(RuntimeError::TypeError("invalid types for -".into())),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, RuntimeError>;
    fn mul(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
            (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 * r)),
            (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l * *r as f64)),
            _ => Err(RuntimeError::TypeError("invalid types for *".into())),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, RuntimeError>;
    fn div(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(_), Value::Int(r)) if *r == 0 => {
                Err(RuntimeError::TypeError("division by zero".into()))
            }
            (Value::Float(_), Value::Float(r)) if *r == 0.0 => {
                Err(RuntimeError::TypeError("division by zero".into()))
            }
            (Value::Int(l), Value::Int(r)) => Ok(Value::Float(*l as f64 / *r as f64)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l / r)),
            (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 / r)),
            (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l / *r as f64)),
            _ => Err(RuntimeError::TypeError("invalid types for /".into())),
        }
    }
}

impl Rem for Value {
    type Output = Result<Value, RuntimeError>;
    fn rem(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(_), Value::Int(r)) if *r == 0 => {
                Err(RuntimeError::TypeError("modulo by zero".into()))
            }
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l % r)),
            _ => Err(RuntimeError::TypeError("% requires integers".into())),
        }
    }
}

impl Neg for Value {
    type Output = Result<Value, RuntimeError>;
    fn neg(self) -> Self::Output {
        match self {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(f) => Ok(Value::Float(-f)),
            _ => Err(RuntimeError::TypeError(
                "unary - requires numeric type".into(),
            )),
        }
    }
}

impl Not for Value {
    type Output = Result<Value, RuntimeError>;
    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Int(n) => Ok(Value::Int(!n)),
            _ => Err(RuntimeError::TypeError("! requires bool or int".into())),
        }
    }
}

impl BitOr for Value {
    type Output = Result<Value, RuntimeError>;
    fn bitor(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l | r)),
            _ => Err(RuntimeError::TypeError("| requires integers".into())),
        }
    }
}

impl BitXor for Value {
    type Output = Result<Value, RuntimeError>;
    fn bitxor(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l ^ r)),
            _ => Err(RuntimeError::TypeError("^ requires integers".into())),
        }
    }
}

impl BitAnd for Value {
    type Output = Result<Value, RuntimeError>;
    fn bitand(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l & r)),
            _ => Err(RuntimeError::TypeError("& requires integers".into())),
        }
    }
}

impl Shl for Value {
    type Output = Result<Value, RuntimeError>;
    fn shl(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l << r)),
            _ => Err(RuntimeError::TypeError("<< requires integers".into())),
        }
    }
}

impl Shr for Value {
    type Output = Result<Value, RuntimeError>;
    fn shr(self, b: Value) -> Self::Output {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l >> r)),
            _ => Err(RuntimeError::TypeError(">> requires integers".into())),
        }
    }
}

impl Value {
    pub fn pow(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int((*l as f64).powf(*r as f64) as i64)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l.powf(*r))),
            (Value::Int(l), Value::Float(r)) => Ok(Value::Float((*l as f64).powf(*r))),
            (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l.powf(*r as f64))),
            _ => Err(RuntimeError::TypeError("** requires numeric types".into())),
        }
    }

    pub fn bitwise_not(self) -> Result<Value, RuntimeError> {
        match self {
            Value::Int(n) => Ok(Value::Int(!n)),
            _ => Err(RuntimeError::TypeError("~ requires integer".into())),
        }
    }

    pub fn logic_and(self, b: Value) -> Result<Value, RuntimeError> {
        Ok(Value::Bool(self.as_bool() && b.as_bool()))
    }

    pub fn logic_or(self, b: Value) -> Result<Value, RuntimeError> {
        Ok(Value::Bool(self.as_bool() || b.as_bool()))
    }

    pub fn val_eq(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), Value::String(_)) => Ok(Value::Bool(self == b)),
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare string with different type".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() == b.as_float())),
        }
    }

    pub fn val_ne(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), Value::String(_)) => Ok(Value::Bool(self != b)),
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare string with different type".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() != b.as_float())),
        }
    }

    pub fn val_lt(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare strings with <".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() < b.as_float())),
        }
    }

    pub fn val_gt(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare strings with >".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() > b.as_float())),
        }
    }

    pub fn val_le(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare strings with <=".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() <= b.as_float())),
        }
    }

    pub fn val_ge(self, b: Value) -> Result<Value, RuntimeError> {
        match (&self, &b) {
            (Value::String(_), _) | (_, Value::String(_)) => Err(RuntimeError::TypeError(
                "cannot compare strings with >=".into(),
            )),
            _ => Ok(Value::Bool(self.as_float() >= b.as_float())),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Float(f) => *f as i64,
            Value::Bool(b) => *b as i64,
            _ => 0,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            _ => false,
        }
    }

    pub fn as_float(&self) -> f64 {
        match self {
            Value::Int(n) => *n as f64,
            Value::Float(f) => *f,
            Value::Bool(b) => *b as i64 as f64,
            _ => 0.0,
        }
    }

    pub fn as_syscall_arg(&self) -> i64 {
        match self {
            Value::String(s) => s.as_ptr() as i64,
            _ => self.as_int(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(_) => write!(f, "{}", self.as_int()),
            Value::Float(_) => write!(f, "{:?}", self.as_float()),
            Value::Bool(_) => write!(f, "{}", self.as_bool()),
            Value::String(s) => write!(f, "{}", *s),
            Value::Array(a) => {
                write!(f, "{{")?;
                for i in 0..a.len() {
                    write!(f, "{}", a[i])?;
                    if i + 1 < a.len() {
                        write!(f, ",")?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}
