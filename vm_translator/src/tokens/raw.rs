use identifier::Identifier;
use number::Number;
use special_char::SpecialChar;

use crate::tokenizer::TryTokenize;

pub mod identifier;
pub mod number;
pub mod special_char;

pub enum RawToken {
    Identifier(Identifier),
    Number(Number),
    SpecialChar(SpecialChar),
}

impl RawToken {
    pub fn tokenize(line: String) -> Vec<RawToken> {
        let mut iter = line.chars().peekable();
        let mut tokens: Vec<RawToken> = vec![];
        while iter.peek().is_some() {
            if let Some(token) = Identifier::try_tokenize(&mut iter) {
                tokens.push(RawToken::Identifier(token));
                continue;
            }

            if let Some(token) = Number::try_tokenize(&mut iter) {
                tokens.push(RawToken::Number(token));
                continue;
            }

            if let Some(token) = SpecialChar::try_tokenize(&mut iter) {
                tokens.push(RawToken::SpecialChar(token));
                continue;
            }
        }

        tokens
    }
}
