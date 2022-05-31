mod ast;
mod lexer;
mod operators;
mod parser;
mod runner;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let ast = parser::parse(&args.file)?;
    runner::run(&ast);

    Ok(())
}
