use crate::{
    ast::{Exp, SeqExp},
    lexer::Token,
};

use logos::{Lexer, Logos};
use std::fs;

#[derive(Default)]
pub struct State {
    pub loop_depth: u8,
}

pub type ParseResult = Result<SeqExp, std::io::Error>;
pub fn parse(path: &str) -> ParseResult {
    let input = fs::read_to_string(path)?;
    let mut lexer = Token::lexer(&input);

    let mut tree = SeqExp::default();
    let mut state = State::default();

    parse_seq_exp(&mut tree, &mut lexer, &mut state);

    Ok(tree)
}

fn parse_seq_exp(node: &mut SeqExp, lexer: &mut Lexer<Token>, state: &mut State) {
    while let Some(token) = lexer.next() {
        match token {
            Token::Left => node.exps.push(Exp::Left),
            Token::Right => node.exps.push(Exp::Right),
            Token::Increment => node.exps.push(Exp::Increment),
            Token::Decrement => node.exps.push(Exp::Decrement),
            Token::Input => node.exps.push(Exp::Input),
            Token::Output => node.exps.push(Exp::Output),
            Token::LoopStart => {
                state.loop_depth += 1;
                let mut body = SeqExp::default();
                parse_seq_exp(&mut body, lexer, state);
                node.exps.push(Exp::Loop { body });
            }
            Token::LoopEnd => {
                if state.loop_depth == 0 {
                    panic!("Loop ending token `{}` unmatched.", lexer.slice());
                }
                state.loop_depth -= 1;
                return;
            }
            _ => eprint!("{}", lexer.slice()),
        }
    }
}
