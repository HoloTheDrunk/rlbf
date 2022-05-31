use crate::{
    ast::{Exp, SeqExp},
    operators::*,
};

pub struct State {
    pub strip: [u8; u8::MAX as usize],
    pub position: u8,
}

impl Default for State {
    fn default() -> Self {
        State {
            strip: [0; u8::MAX as usize],
            position: 0,
        }
    }
}

pub fn run(ast: &SeqExp) {
    let mut state = State::default();
    run_seq_exp(ast, &mut state);
}

fn run_seq_exp(node: &SeqExp, state: &mut State) {
    for exp in node.exps.iter() {
        run_exp(exp, state);
        if !matches!(exp, Exp::Loop { body: _ }) {
            eprintln!(
                "{}: {:?}",
                exp,
                state.strip[0..10].iter().collect::<Vec<&u8>>()
            );
        }
    }
}

fn run_exp(node: &Exp, state: &mut State) {
    match node {
        Exp::Left => left(state),
        Exp::Right => right(state),
        Exp::Increment => increment(state),
        Exp::Decrement => decrement(state),
        Exp::Output => output(state),
        Exp::Input => input(state),
        Exp::Loop { body } => {
            while *state.strip.get(state.position as usize).unwrap() != 0 {
                run_seq_exp(body, state);
            }
        },
    }
}
