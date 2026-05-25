use crate::eval::{BuiltinFn, Environment};
use crate::value::Value;
use libc::syscall;
use usr_input::input;

fn c_print(args: Vec<Value>) -> Value {
    for arg in args {
        print!("{}", arg);
    }
    Value::Int(0)
}

fn c_println(args: Vec<Value>) -> Value {
    c_print(args);
    println!();
    Value::Int(0)
}

fn c_input(args: Vec<Value>) -> Value {
    let mut buf = String::new();
    input!(buf, "{}", args[0]).expect("Failed to get input");
    buf.truncate(buf.trim_end().len());
    Value::String(buf)
}

fn c_strlen(args: Vec<Value>) -> Value {
    if let Value::String(str) = &args[0] {
        Value::Int(str.len() as i64)
    } else {
        Value::Int(0)
    }
}

fn c_typeof(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Int(_) => Value::String("int".to_string()),
        Value::Float(_) => Value::String("float".to_string()),
        Value::String(_) => Value::String("string".to_string()),
        Value::Bool(_) => Value::String("bool".to_string()),
        Value::Array(_) => Value::String("array".to_string()),
    }
}

fn c_platformid(_: Vec<Value>) -> Value {
    Value::String(std::env::consts::OS.to_string())
}

fn c_syscall(args: Vec<Value>) -> Value {
    let num = args[0].as_int() as i32;
    let arg = |n: usize| args[n].as_syscall_arg();

    let ret = unsafe {
        match args.len() {
            1 => syscall(num),
            2 => syscall(num, arg(1)),
            3 => syscall(num, arg(1), arg(2)),
            4 => syscall(num, arg(1), arg(2), arg(3)),
            5 => syscall(num, arg(1), arg(2), arg(3), arg(4)),
            6 => syscall(num, arg(1), arg(2), arg(3), arg(4), arg(5)),
            7 => syscall(num, arg(1), arg(2), arg(3), arg(4), arg(5), arg(6)),
            _ => panic!("syscall takes at most 7 arguments"),
        }
    };

    Value::Int(ret as i64)
}

fn c_sin(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().sin())
}

fn c_cos(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().cos())
}

fn c_tan(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().tan())
}

fn c_abs(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().abs())
}

fn c_sqrt(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().sqrt())
}

fn c_log(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().log2())
}

fn c_log10(args: Vec<Value>) -> Value {
    Value::Float(args[0].as_float().log10())
}

macro_rules! mkbpair {
    ($name:literal, $nargs:expr, $fn:ident) => {
        (
            $name.to_string(),
            BuiltinFn {
                nargs: $nargs,
                fnc: $fn,
            },
        )
    };
}

pub fn build_env() -> Option<Environment> {
    let mut env = Environment::default();

    env.bfnlut.extend([
        mkbpair!("print", -1, c_print),
        mkbpair!("println", -1, c_println),
        mkbpair!("input", 1, c_input),
        mkbpair!("syscall", -1, c_syscall),
        mkbpair!("strlen", 1, c_strlen),
        mkbpair!("typeof", 1, c_typeof),
        mkbpair!("platformid", 0, c_platformid),
        mkbpair!("sin", 1, c_sin),
        mkbpair!("cos", 1, c_cos),
        mkbpair!("tan", 1, c_tan),
        mkbpair!("abs", 1, c_abs),
        mkbpair!("sqrt", 1, c_sqrt),
        mkbpair!("log", 1, c_log),
        mkbpair!("log10", 1, c_log10),
    ]);

    Some(env)
}
