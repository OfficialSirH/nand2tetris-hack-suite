use std::{collections::HashSet, iter::Peekable};

use crate::tokens::{Token, raw::RawToken};

pub type SymbolsTable = HashSet<String>;

pub struct VMTranslator<I> {
    symbols_table: SymbolsTable,
    iter: I,
}

impl<I: Iterator<Item = Token>> VMTranslator<I> {
    pub fn tokenize(vec: Vec<String>) -> VMTranslator<impl Iterator<Item = Token>> {
        let mut symbols_table = SymbolsTable::new();
        VMTranslator {
            symbols_table,
            iter: vec
                .into_iter()
                .filter_map(Self::tokenize_line)
                .map(Token::parse),
        }
    }

    pub fn translate(iter: impl Iterator<Item = Token>) -> ! {
        todo!()
    }

    pub fn tokenize_line(line: String) -> Option<impl Iterator<Item = RawToken>> {
        if line.is_empty() {
            return None;
        }

        let tokens = RawToken::tokenize(line);

        Some(tokens.into_iter())
    }
}

pub trait Tokenize {
    fn tokenize(value: String) -> Self;
}

pub trait TryTokenize {
    fn try_tokenize<'iter, I: Iterator<Item = char>>(iter: &'iter mut Peekable<I>) -> Option<Self>
    where
        Self: Sized;
}

pub trait TryParse {
    fn try_parse(value: &str, iter: &mut impl Iterator<Item = RawToken>) -> Option<Self>
    where
        Self: Sized;
}
