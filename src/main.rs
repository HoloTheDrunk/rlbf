mod ast;
mod compiler;
use compiler::Compiler;
mod lexer;
mod operators;
mod parser;
mod runner;

use std::process::Command;
use clap::Parser;
use inkwell::context::Context;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source Brainfuck file path
    #[clap(short, long)]
    input: String,

    /// If specified, compile program with given name (requires llvm)
    #[clap(short, long)]
    output: Option<String>,

    /// If specified, run gcc on generated object files (requires -o/--output)
    #[clap(short, long)]
    full: bool,

    /// If specified, automatically run output program (requires -f/--full)
    #[clap(short, long)]
    run: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let ast = parser::parse(&args.input)?;

    if let Some(output) = &args.output {
        let object_file = format!("{}.o", output);

        Compiler::init_targets();

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

        if let Err(err) = compiler.write_to_file(object_file.as_str()) {
            eprintln!(
                "An error happend while saving the output to `{}`: {}",
                output, err
            );
        }

        // Compile program with gcc
        if args.full {
            Command::new("gcc")
                .args([object_file.as_str(), "-o", output])
                .output()
                .expect("Failed to compile objects. Maybe missing gcc?");

            // Remove object file
            std::fs::remove_file(object_file)?;

            // Run program and print output on stdout
            if args.run {
                let stdout = Command::new(format!("./{}", output))
                    .output()
                    .expect("Failed to run output program")
                    .stdout;

                println!("{}", std::str::from_utf8(&stdout).unwrap());
            }
        }
    } else {
        runner::run(&ast);
    }

    Ok(())
}
