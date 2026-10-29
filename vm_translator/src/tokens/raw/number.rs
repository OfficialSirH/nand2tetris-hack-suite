use utils::ToPeekingTakeWhile;

use crate::tokenizer::{Tokenize, TryTokenize};

pub struct Number(pub u16);

impl Number {
    fn number(char: &char) -> bool {
        char.is_ascii_digit()
    }
}

impl Tokenize for Number {
    fn tokenize(value: String) -> Self {
        Number(
            u16::from_str_radix(value.as_str(), 10)
                .expect("Number validity should be checked by tokenizer"),
        )
    }
}

impl TryTokenize for Number {
    fn try_tokenize<'iter, I: Iterator<Item = char>>(
        iter: &'iter mut std::iter::Peekable<I>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::number(iter.peek()?) {
            return Some(Number::tokenize(
                iter.peeking_take_while(Self::number).collect(),
            ));
        }

        None
    }
}
