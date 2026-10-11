use crate::tokens::{
    AInstruction, CInstruction, CToken, Delimiter, Function, Instruction as TokenizedInstruction,
    Jump, Value,
};

pub fn parse(instructions: Vec<TokenizedInstruction>) -> Vec<Instruction> {
    instructions.into_iter().map(Instruction::parse).collect()
}

pub trait Parse<T> {
    fn parse(value: T) -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Address(u16),
    Control(Control),
}

impl Parse<TokenizedInstruction> for Instruction {
    fn parse(value: TokenizedInstruction) -> Self {
        match value {
            TokenizedInstruction::A(AInstruction(val)) => Self::Address(val),
            TokenizedInstruction::C(val) => Self::Control(Control::parse(val)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Control {
    pub destination: Option<Destination>,
    pub compute: Compute,
    pub jump: Option<Jump>,
}

impl Parse<CInstruction> for Control {
    fn parse(CInstruction(tokens): CInstruction) -> Self {
        let has_dest = tokens.contains(&CToken::Delimiter(Delimiter::Assign));

        let mut tokens = tokens.into_iter();

        let destination = has_dest.then(|| Self::parse_destination(&mut tokens));
        let (compute, has_jump) = Self::parse_compute(&mut tokens);
        let jump = has_jump.then(|| Self::parse_jump(&mut tokens));

        if tokens.count() > 0 {
            panic!("Unexpected tokens after C Instruction");
        }

        Self {
            destination,
            compute,
            jump,
        }
    }
}

impl Control {
    fn parse_destination(tokens: &mut impl Iterator<Item = CToken>) -> Destination {
        let mut destination_parts = Vec::new();
        let mut push_dest = |val: Value| {
            let dest = DestinationPart::from(val);
            if destination_parts.contains(&dest) {
                panic!("Duplicate destination found");
            }
            destination_parts.push(dest);
        };
        while let Some(token) = tokens.next() {
            match token {
                CToken::Value(val @ (Value::A | Value::D | Value::M)) => push_dest(val),
                CToken::Delimiter(Delimiter::Assign) => {
                    return Destination::from(destination_parts);
                }
                _ => panic!("Unexpected token in destination"),
            }
        }

        unreachable!("The destination loop always return")
    }

    fn parse_compute(tokens: &mut impl Iterator<Item = CToken>) -> (Compute, bool) {
        let mut has_jump = false;
        let mut tokens = tokens.take_while(|token| {
            if token == &CToken::Delimiter(Delimiter::Semicolon) {
                has_jump = true;
                return false;
            }
            true
        });
        let compute = Compute::parse(&mut tokens);
        // A hacky solution to ensure that take_while's bool toggle occurs
        let _ = tokens.next();

        (compute, has_jump)
    }

    fn parse_jump(tokens: &mut impl Iterator<Item = CToken>) -> Jump {
        let CToken::Jump(jump) = tokens.next().expect("Expected Jump token after Delimiter") else {
            panic!("Unexpected token after Jump Delimiter");
        };

        jump
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DestinationPart {
    A,
    D,
    M,
}

impl From<Value> for DestinationPart {
    fn from(value: Value) -> Self {
        match value {
            Value::A => DestinationPart::A,
            Value::D => DestinationPart::D,
            Value::M => DestinationPart::M,
            _ => panic!("Invalid destination token"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Destination {
    A,
    D,
    M,
    AD,
    AM,
    DM,
    ADM,
}

impl From<Vec<DestinationPart>> for Destination {
    fn from(value: Vec<DestinationPart>) -> Self {
        let has_a = value.contains(&DestinationPart::A);
        let has_d = value.contains(&DestinationPart::D);
        let has_m = value.contains(&DestinationPart::M);
        match (has_a, has_d, has_m) {
            (true, false, false) => Destination::A,
            (false, true, false) => Destination::D,
            (false, false, true) => Destination::M,
            (true, true, false) => Destination::AD,
            (true, false, true) => Destination::AM,
            (false, true, true) => Destination::DM,
            (true, true, true) => Destination::ADM,
            (false, false, false) => panic!("Invalid destination parts"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compute {
    Zero,
    One,
    NegativeOne,
    D,
    AorM(bool),
    NotD,
    NotAorM(bool),
    NegativeD,
    NegativeAorM(bool),
    DPlusOne,
    AorMPlusOne(bool),
    DMinusOne,
    AorMMinusOne(bool),
    DPlusAorM(bool),
    DMinusAorM(bool),
    AorMMinusD(bool),
    DANDAorM(bool),
    DORAorM(bool),
}

impl<T> Parse<&mut T> for Compute
where
    T: Iterator<Item = CToken>,
{
    fn parse(value: &mut T) -> Self {
        match value.next().expect("Missing compute tokens") {
            CToken::Value(val) => Self::parse_value(value, val),
            CToken::Function(fun) => Self::parse_prefix_function(value, fun),
            _ => panic!("Invalid token in compute"),
        }
    }
}

impl Compute {
    fn parse_value(tokens: &mut impl Iterator<Item = CToken>, value: Value) -> Compute {
        if Value::Zero == value {
            return Compute::Zero;
        }

        if let Some(token) = tokens.next() {
            let CToken::Function(function) = token else {
                panic!("Unexpected token after value: {token:#?}");
            };

            let CToken::Value(rh_value) = tokens.next().expect("Missing value after function")
            else {
                panic!("Unexpected token after function");
            };
            return match function {
                Function::Add => Self::parse_add(value, rh_value),
                Function::Subtract => Self::parse_subtract(value, rh_value),
                Function::And => Self::parse_and(value, rh_value),
                Function::Or => Self::parse_or(value, rh_value),
                Function::Not => panic!("NOT function is not an infix operator"),
            };
        }

        match value {
            Value::A => Compute::AorM(false),
            Value::D => Compute::D,
            Value::M => Compute::AorM(true),
            Value::One => Compute::One,
            Value::Zero => unreachable!("Zero is always checked at the beginning of the function"),
        }
    }

    fn parse_add(lh: Value, rh: Value) -> Compute {
        match (lh, rh) {
            (Value::A, Value::D) | (Value::D, Value::A) => Compute::DPlusAorM(false),
            (Value::M, Value::D) | (Value::D, Value::M) => Compute::DPlusAorM(true),
            (Value::A, Value::One) | (Value::One, Value::A) => Compute::AorMPlusOne(false),
            (Value::M, Value::One) | (Value::One, Value::M) => Compute::AorMPlusOne(true),
            (Value::D, Value::One) | (Value::One, Value::D) => Compute::DPlusOne,
            _ => panic!("Invalid addition value pair"),
        }
    }

    fn parse_subtract(lh: Value, rh: Value) -> Compute {
        match (lh, rh) {
            (Value::A, Value::D) => Compute::AorMMinusD(false),
            (Value::M, Value::D) => Compute::AorMMinusD(true),
            (Value::D, Value::A) => Compute::DMinusAorM(false),
            (Value::D, Value::M) => Compute::DMinusAorM(true),
            (Value::A, Value::One) => Compute::AorMMinusOne(false),
            (Value::M, Value::One) => Compute::AorMMinusOne(true),
            (Value::D, Value::One) => Compute::DMinusOne,
            _ => panic!("Invalid subtraction value pair"),
        }
    }

    fn parse_and(lh: Value, rh: Value) -> Compute {
        match (lh, rh) {
            (Value::D, Value::A) | (Value::A, Value::D) => Compute::DANDAorM(false),
            (Value::D, Value::M) | (Value::M, Value::D) => Compute::DANDAorM(true),
            _ => panic!("Invalid AND value pair"),
        }
    }

    fn parse_or(lh: Value, rh: Value) -> Compute {
        match (lh, rh) {
            (Value::D, Value::A) | (Value::A, Value::D) => Compute::DORAorM(false),
            (Value::D, Value::M) | (Value::M, Value::D) => Compute::DORAorM(true),
            _ => panic!("Invalid OR value pair"),
        }
    }

    fn parse_prefix_function(
        tokens: &mut impl Iterator<Item = CToken>,
        function: Function,
    ) -> Compute {
        let compute = match function {
            Function::Subtract => Self::parse_negatable_value(tokens.next()),
            Function::Not => Self::parse_nottable_value(tokens.next()),
            _ => panic!("Unexpected function at start of compute"),
        };

        compute
    }

    fn parse_negatable_value(token: Option<CToken>) -> Compute {
        let CToken::Value(value) = token.expect("Missing value for negation") else {
            panic!("Invalid token after negation");
        };
        match value {
            Value::A => Compute::NegativeAorM(false),
            Value::D => Compute::NegativeD,
            Value::M => Compute::NegativeAorM(true),
            Value::One => Compute::NegativeOne,
            Value::Zero => panic!("Why are you negating 0?"),
        }
    }

    fn parse_nottable_value(token: Option<CToken>) -> Compute {
        let CToken::Value(value) = token.expect("Missing value for NOT") else {
            panic!("Invalid token after NOT");
        };
        match value {
            Value::A => Compute::NotAorM(false),
            Value::D => Compute::NotD,
            Value::M => Compute::NotAorM(true),
            Value::One | Value::Zero => panic!("Why are you NOTing constants?"),
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{
        parser::{self, Compute, Control, Destination, Instruction},
        tokenizer::Tokenizer,
        tokens::{AInstruction, Instruction as TokenizedInstruction, Jump},
        utils::multiline_to_clean_vec,
    };

    #[test]
    fn parse_a() {
        let instructions = vec![
            TokenizedInstruction::A(AInstruction(17)),
            TokenizedInstruction::A(AInstruction(39)),
        ];
        let parsed_instructions = vec![Instruction::Address(17), Instruction::Address(39)];
        assert_eq!(parser::parse(instructions), parsed_instructions);
    }

    #[test]
    fn parse_c() {
        let mut tokenizer = Tokenizer::new();
        let instructions = parser::parse(tokenizer.tokenize(multiline_to_clean_vec(
            "
            A
            D=1
            M=A+1
            ADM=!D;JMP
            ",
        )));
        let parsed_instructions = vec![
            Instruction::Control(Control {
                destination: None,
                compute: Compute::AorM(false),
                jump: None,
            }),
            Instruction::Control(Control {
                destination: Some(Destination::D),
                compute: Compute::One,
                jump: None,
            }),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::AorMPlusOne(false),
                jump: None,
            }),
            Instruction::Control(Control {
                destination: Some(Destination::ADM),
                compute: Compute::NotD,
                jump: Some(Jump::JMP),
            }),
        ];
        assert_eq!(instructions, parsed_instructions);
    }

    #[test]
    fn parse_example_program() {
        let mut tokenizer = Tokenizer::new();
        let instructions = parser::parse(tokenizer.tokenize(multiline_to_clean_vec(
            "
                @sum
                M=0
                @from
                M=1
                @10
                D=A
                @to
                M=D
                (LOOP)
                @from
                D=M
                M=M+1
                @sum
                M=M+D
                @from
                D=M
                @to
                D=M-D
                @LOOP
                D;JLT
            ",
        )));
        let expected_instructions = vec![
            Instruction::Address(16),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::Zero,
                jump: None,
            }),
            Instruction::Address(17),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::One,
                jump: None,
            }),
            Instruction::Address(10),
            Instruction::Control(Control {
                destination: Some(Destination::D),
                compute: Compute::AorM(false),
                jump: None,
            }),
            Instruction::Address(18),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::D,
                jump: None,
            }),
            Instruction::Address(17),
            Instruction::Control(Control {
                destination: Some(Destination::D),
                compute: Compute::AorM(true),
                jump: None,
            }),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::AorMPlusOne(true),
                jump: None,
            }),
            Instruction::Address(16),
            Instruction::Control(Control {
                destination: Some(Destination::M),
                compute: Compute::DPlusAorM(true),
                jump: None,
            }),
            Instruction::Address(17),
            Instruction::Control(Control {
                destination: Some(Destination::D),
                compute: Compute::AorM(true),
                jump: None,
            }),
            Instruction::Address(18),
            Instruction::Control(Control {
                destination: Some(Destination::D),
                compute: Compute::AorMMinusD(true),
                jump: None,
            }),
            Instruction::Address(8),
            Instruction::Control(Control {
                destination: None,
                compute: Compute::D,
                jump: Some(Jump::JLT),
            }),
        ];
        assert_eq!(instructions, expected_instructions);
    }
}
