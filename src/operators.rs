use crate::runner::State;
use std::io::Read;

pub fn left(state: &mut State) {
    state.position = state.position.wrapping_sub(1);
}

pub fn right(state: &mut State) {
    state.position = state.position.wrapping_add(1);
}

pub fn increment(state: &mut State) {
    let cell = state.strip.get_mut(state.position as usize).unwrap();
    *cell = cell.wrapping_add(1);
}

pub fn decrement(state: &mut State) {
    let cell = state.strip.get_mut(state.position as usize).unwrap();
    *cell = cell.wrapping_sub(1);
}

pub fn output(state: &State) {
    print!(
        "{}",
        *state.strip.get(state.position as usize).unwrap() as char
    );
}

pub fn input(state: &mut State) {
    if let Some(value) = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
    {
        let cell = state.strip.get_mut(state.position as usize).unwrap();
        *cell = value;
    }
}
