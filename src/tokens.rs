use crate::tokenizer::{MaybeTokenize, Tokenize, Tokenizer};
use std::{iter::Peekable, str::Chars};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Instruction {
    /// The A Instruction sets the A register and is defined as the following:
    /// # Assembly
    /// ## Direct memory access
    /// @[0-16383]
    /// ## Automaticaly defined memory access via named variables
    /// @[A-Z]
    /// # Binary
    /// 0xxxxxxxxxxxxxxx
    /// **x = bit values for the memory address being accessed**
    A(AInstruction),
    C(CInstruction),
}

impl Tokenize<String> for Instruction {
    fn tokenize(ctx: &mut Tokenizer, value: String) -> Self {
        let mut chars = value.chars().peekable();
        match chars.peek() {
            Some('@') => {
                chars.next();
                Instruction::A(AInstruction::tokenize(ctx, chars.collect::<String>()))
            }
            Some(_) => Instruction::C(CInstruction::tokenize(ctx, chars)),

            _ => panic!("Unexpected empty instruction"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AInstruction(pub u16);

impl Tokenize<String> for AInstruction {
    fn tokenize(ctx: &mut Tokenizer, value: String) -> Self {
        if let Ok(memory) = u16::from_str_radix(value.as_str(), 10) {
            return Self(memory);
        }

        let memory = ctx.upsert_symbols_table(value);
        Self(memory)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CInstruction(pub Vec<CToken>);

impl Tokenize<Peekable<Chars<'_>>> for CInstruction {
    fn tokenize(ctx: &mut Tokenizer, mut value: Peekable<Chars>) -> Self {
        let mut tokens = Vec::new();
        while let Some(char) = value.next() {
            let token = CToken::tokenize(ctx, char);
            if let CToken::Delimiter(Delimiter::Semicolon) = token {
                tokens.push(token);
                let jmp = Jump::tokenize(ctx, value.collect::<String>());
                tokens.push(CToken::Jump(jmp));
                break;
            }
            tokens.push(token);
        }

        CInstruction(tokens)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CToken {
    Value(Value),
    Function(Function),
    Jump(Jump),
    Delimiter(Delimiter),
}

impl Tokenize<char> for CToken {
    fn tokenize(ctx: &mut Tokenizer, value: char) -> Self {
        if let Some(val) = Value::maybe_tokenize(ctx, value) {
            return Self::Value(val);
        }

        if let Some(val) = Function::maybe_tokenize(ctx, value) {
            return Self::Function(val);
        }

        Self::Delimiter(Delimiter::tokenize(ctx, value))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Value {
    A,
    D,
    M,
    Zero,
    One,
}

impl MaybeTokenize<char> for Value {
    fn maybe_tokenize(_: &mut Tokenizer, value: char) -> Option<Self> {
        match value {
            'A' => Some(Self::A),
            'D' => Some(Self::D),
            'M' => Some(Self::M),
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Function {
    Add,
    Subtract,
    And,
    Or,
    Not,
}

impl MaybeTokenize<char> for Function {
    fn maybe_tokenize(_: &mut Tokenizer, value: char) -> Option<Self> {
        match value {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '&' => Some(Self::And),
            '|' => Some(Self::Or),
            '!' => Some(Self::Not),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Delimiter {
    Assign,
    Semicolon,
}

impl Tokenize<char> for Delimiter {
    fn tokenize(_: &mut Tokenizer, value: char) -> Self {
        match value {
            '=' => Self::Assign,
            ';' => Self::Semicolon,
            _ => panic!("Unexpected C Instruction Token: {value}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Tokenize<String> for Jump {
    fn tokenize(_: &mut Tokenizer, value: String) -> Self {
        match value.as_str() {
            "JGT" => Jump::JGT,
            "JEQ" => Jump::JEQ,
            "JGE" => Jump::JGE,
            "JLT" => Jump::JLT,
            "JNE" => Jump::JNE,
            "JLE" => Jump::JLE,
            "JMP" => Jump::JMP,
            _ => panic!("Unexpected jump instruction"),
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use crate::{
        tokenizer::{Tokenize, Tokenizer},
        tokens::{
            AInstruction, CInstruction, CToken, Delimiter, Function, Instruction, Jump, Value,
        },
    };

    #[test]
    fn a_instructions_constants() {
        let mut tokenizer = Tokenizer::new();

        let instruction = String::from("@17");
        assert_eq!(
            Instruction::A(AInstruction(17_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );

        let instruction = String::from("@16383");
        assert_eq!(
            Instruction::A(AInstruction(16383_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );
    }

    #[test]
    fn a_instructions_variables() {
        let mut tokenizer = Tokenizer::new();

        let instruction = String::from("@var");
        assert_eq!(
            Instruction::A(AInstruction(16_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );

        let instruction = String::from("@var2");
        assert_eq!(
            Instruction::A(AInstruction(17_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );

        let instruction = String::from("@var");
        assert_eq!(
            Instruction::A(AInstruction(16_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );

        let instruction = String::from("@var3");
        assert_eq!(
            Instruction::A(AInstruction(18_u16)),
            Instruction::tokenize(&mut tokenizer, instruction)
        );
    }

    #[test]
    fn c_instructions() {
        let mut tokenizer = Tokenizer::new();

        let instruction = String::from("A=D+M;JMP");
        assert_eq!(
            Instruction::C(CInstruction(vec![
                CToken::Value(Value::A),
                CToken::Delimiter(Delimiter::Assign),
                CToken::Value(Value::D),
                CToken::Function(Function::Add),
                CToken::Value(Value::M),
                CToken::Delimiter(Delimiter::Semicolon),
                CToken::Jump(Jump::JMP),
            ])),
            Instruction::tokenize(&mut tokenizer, instruction)
        );
    }
}
