pub mod ast;
pub mod ast_expr;
pub mod ast_stmt;
pub mod build_env;
pub mod error;
pub mod eval;
pub mod eval_expr;
pub mod eval_stmt;
pub mod tokenizer;
pub mod value;

use clap::Parser;
use std::fs::read_to_string;

use crate::{ast::parse, eval::eval, tokenizer::tokenize};

#[derive(Parser)]
struct Cli {
    #[arg(help = "Path of the source file")]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let srcbuf = read_to_string(cli.file)?;
    let tokens = tokenize(srcbuf)?;
    let ast = parse(tokens)?;
    eval(ast)?;

    Ok(())
}
