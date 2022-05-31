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
    #[regex(r"[ a-zA-Z0-9\t\n\f]+", logos::skip)]
    Error,
}
