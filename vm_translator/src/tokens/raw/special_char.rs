use crate::tokenizer::TryTokenize;

pub enum SpecialChar {
    Colon,
    LeftParens,
    RightParens,
}

impl TryTokenize for SpecialChar {
    fn try_tokenize<'iter, I: Iterator<Item = char>>(
        iter: &'iter mut std::iter::Peekable<I>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        match iter.peek()? {
            ':' => Some(SpecialChar::Colon),
            '(' => Some(SpecialChar::LeftParens),
            ')' => Some(SpecialChar::RightParens),
            _ => None,
        }
    }
}
