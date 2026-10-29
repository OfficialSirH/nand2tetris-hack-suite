use crate::{
    tokenizer::{SymbolsTable, TryParse},
    tokens::{
        arithmetic::Arithmetic,
        branching::Branching,
        function::Function,
        raw::{RawToken, identifier},
        stack::Stack,
    },
};

pub mod arithmetic;
pub mod branching;
pub mod function;
pub mod raw;
pub mod stack;

pub enum Token {
    Arithmetic(arithmetic::Arithmetic),
    Branching(branching::Branching),
    Function(function::Function),
    Stack(stack::Stack),
}

impl Token {
    pub fn parse(mut line: impl Iterator<Item = RawToken>) -> Token {
        let RawToken::Identifier(identifier::Identifier(identifier)) = line
            .next()
            .expect("There should be at least one RawToken in a line")
        else {
            panic!("Unexpected token at start of line");
        };
        let token = Self::parse_identifier(&mut line, identifier);

        if line.count() > 0 {
            panic!("Unexpected tokens found at end of line");
        }

        token
    }

    fn parse_identifier(iter: &mut impl Iterator<Item = RawToken>, value: String) -> Token {
        let value = value.as_str();

        if let Some(arithmetic) = Arithmetic::try_parse(value, iter) {
            return Token::Arithmetic(arithmetic);
        }

        if let Some(branching) = Branching::try_parse(value, iter) {
            return Token::Branching(branching);
        }

        if let Some(function) = Function::try_parse(value, iter) {
            return Token::Function(function);
        }

        if let Some(stack) = Stack::try_parse(value, iter) {
            return Token::Stack(stack);
        }

        todo!("variables and function names")
    }
}

#[cfg(test)]
mod test {
    use std::array::IntoIter;

    use crate::tokens::{
        Token,
        raw::{RawToken, identifier::Identifier, number::Number},
    };

    #[test]
    fn parse() {
        let tokens = vec![
            vec![
                RawToken::Identifier(Identifier(String::from("push"))),
                RawToken::Number(Number(1)),
            ],
            vec![
                RawToken::Identifier(Identifier(String::from("push"))),
                RawToken::Number(Number(2)),
            ],
            vec![RawToken::Identifier(Identifier(String::from("add")))],
        ];

        let tokens = tokens
            .into_iter()
            .map(IntoIterator::into_iter)
            .map(Token::parse);

        let expected_tokens = vec![];

        assert_eq!();
    }
}
