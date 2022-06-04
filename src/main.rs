mod ast;
mod lexer;
mod operators;
mod parser;
mod runner;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source Brainfuck file path
    #[clap(short, long)]
    file: String,

    /// If specified, compile program to file at given path (requires llvm)
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let ast = parser::parse(&args.file)?;
    runner::run(&ast);

    if let Some(output) = &args.output {
        dbg!(output);
    }

    Ok(())
}
