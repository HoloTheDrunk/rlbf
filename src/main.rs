mod ast;
mod lexer;
mod parser;

fn main() -> std::io::Result<()> {
    let ast = parser::parse("bf/hello_world.bf")?;

    dbg!(&ast);
    println!("{}", ast);

    Ok(())
}
