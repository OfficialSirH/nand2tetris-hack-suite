use std::iter::Peekable;

use utils::ToPeekingTakeWhile;

use crate::tokenizer::{Tokenize, TryTokenize};

pub struct Identifier(pub String);

impl Identifier {
    fn identifier(char: &char) -> bool {
        char.is_ascii_alphabetic() || *char == '_' || *char == '-'
    }

    fn identifier_after_first_char(char: &char) -> bool {
        Self::identifier(char) || char.is_ascii_digit()
    }
}

impl Tokenize for Identifier {
    fn tokenize(value: String) -> Self {
        Identifier(value)
    }
}

impl TryTokenize for Identifier {
    fn try_tokenize<'iter, I: Iterator<Item = char>>(iter: &'iter mut Peekable<I>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::identifier(iter.peek()?) {
            return Some(Identifier(
                iter.peeking_take_while(Self::identifier_after_first_char)
                    .collect::<String>(),
            ));
        }

        None
    }
}
