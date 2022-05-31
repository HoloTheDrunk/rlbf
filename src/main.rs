mod ast;
mod lexer;
mod operators;
mod parser;
mod runner;

fn main() -> std::io::Result<()> {
    let ast = parser::parse("bf/hello_world.bf")?;
    runner::run(&ast);

    Ok(())
}
