use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("<")]
    Left,
    #[token(">")]
    Right,

    #[token("+")]
    Increment,
    #[token("-")]
    Decrement,

    #[token(".")]
    Output,
    #[token(",")]
    Input,

    #[token("[")]
    LoopStart,
    #[token("]")]
    LoopEnd,

    #[error]
    // Ignore everything that isn't an operator
    #[regex(r"[^<>+-.,\[\]]+", logos::skip)]
    Error,
}
