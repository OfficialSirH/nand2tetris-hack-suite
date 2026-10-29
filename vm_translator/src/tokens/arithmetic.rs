use crate::{tokenizer::TryParse, tokens::raw::RawToken};

pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl TryParse for Arithmetic {
    fn try_parse(value: &str, _: &mut impl Iterator<Item = RawToken>) -> Option<Self>
    where
        Self: Sized,
    {
        use Arithmetic::*;
        match value {
            "add" => Some(Add),
            "sub" => Some(Sub),
            "neg" => Some(Neg),
            "eq" => Some(Eq),
            "gt" => Some(Gt),
            "lt" => Some(Lt),
            "and" => Some(And),
            "or" => Some(Or),
            "not" => Some(Not),
            _ => None,
        }
    }
}
