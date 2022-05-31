use std::fmt::Display;

#[derive(Default, Debug)]
pub struct SeqExp {
    pub exps: Vec<Exp>,
}

impl Display for SeqExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for exp in self.exps.iter() {
            write!(f, "{}", exp)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Exp {
    Left,
    Right,
    Increment,
    Decrement,
    Input,
    Output,
    Loop { body: SeqExp },
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Exp::Left => write!(f, "<"),
            Exp::Right => write!(f, ">"),
            Exp::Increment => write!(f, "+"),
            Exp::Decrement => write!(f, "-"),
            Exp::Input => write!(f, ","),
            Exp::Output => writeln!(f, "."),
            Exp::Loop{body} => write!(f, "[{}]", body),
        }
    }
}
