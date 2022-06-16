mod ast;
mod compiler;
use compiler::Compiler;
mod lexer;
mod operators;
mod parser;
mod runner;

use clap::Parser;
use inkwell::context::Context;

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
        let context = Context::create();
        let compiler = {
            Compiler {
                context: &context,
                module: context.create_module("brainfuck_rust"),
                builder: context.create_builder(),
            }
        };

        if let Err(e) = compiler.compile(&ast) {
            eprintln!("An error happened while compiling the code: {}", e);
        }

        if let Err(err) = compiler.write_to_file(output) {
            eprintln!(
                "An error happend while saving the output to `{}`: {}",
                output, err
            );
        }
    }

    Ok(())
}
