use std::ops::{BitOr, BitOrAssign};

use crate::{
    parser::{Compute, Control, Destination, Instruction},
    tokens::Jump,
};

pub fn generate(assembly: Vec<Instruction>) -> Vec<u8> {
    assembly
        .into_iter()
        .flat_map(|code| code.translate().to_be_bytes())
        .collect()
}

pub trait Code {
    fn translate(self) -> u16;
}

impl Code for Instruction {
    fn translate(self) -> u16 {
        match self {
            Self::Address(val) => val,
            Self::Control(control) => control.translate(),
        }
    }
}

impl Code for Control {
    /// 111accccccdddjjj
    fn translate(self) -> u16 {
        let mut code = 0b1110_0000_0000_0000.bitor(self.compute.translate());

        if let Some(destination) = self.destination {
            code.bitor_assign(destination.translate());
        }

        if let Some(jump) = self.jump {
            code.bitor_assign(jump.translate());
        }

        code
    }
}

impl Code for Compute {
    fn translate(self) -> u16 {
        use Compute::*;
        let (toggle_m, mut control_bits) = match self {
            Zero => (false, 0b0010_1010),
            One => (false, 0b0011_1111),
            NegativeOne => (false, 0b0011_1010),
            D => (false, 0b0000_1100),
            AorM(m) => (m, 0b0011_0000),
            NotD => (false, 0b0000_1101),
            NotAorM(m) => (m, 0b0011_0001),
            NegativeD => (false, 0b0000_1111),
            NegativeAorM(m) => (m, 0b0011_0011),
            DPlusOne => (false, 0b0001_1111),
            AorMPlusOne(m) => (m, 0b0011_0111),
            DMinusOne => (false, 0b0000_1110),
            AorMMinusOne(m) => (m, 0b0011_0010),
            DPlusAorM(m) => (m, 0b0000_0010),
            DMinusAorM(m) => (m, 0b0001_0011),
            AorMMinusD(m) => (m, 0b0000_0111),
            DANDAorM(m) => (m, 0b0000_0000),
            DORAorM(m) => (m, 0b0001_0101),
        };

        if toggle_m {
            control_bits.bitor_assign(0b0100_0000);
        }
        control_bits << 6
    }
}

impl Code for Destination {
    fn translate(self) -> u16 {
        use Destination::*;
        let destination_bits = match self {
            M => 0b0001,
            D => 0b0010,
            DM => 0b0011,
            A => 0b0100,
            AM => 0b0101,
            AD => 0b0110,
            ADM => 0b0111,
        };

        destination_bits << 3
    }
}

impl Code for Jump {
    fn translate(self) -> u16 {
        use Jump::*;
        match self {
            JGT => 0b0001,
            JEQ => 0b0010,
            JGE => 0b0011,
            JLT => 0b0100,
            JNE => 0b0101,
            JLE => 0b0110,
            JMP => 0b0111,
        }
    }
}

#[cfg(test)]
mod code_tests {
    use crate::{code, parser, tokenizer::Tokenizer, utils::multiline_to_clean_vec};

    #[test]
    fn sumation_1_to_10() {
        let mut tokenizer = Tokenizer::new();
        let code = code::generate(parser::parse(tokenizer.tokenize(multiline_to_clean_vec(
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
M=D+M
@from
D=M
@to
D=M-D
@LOOP
D;JGE
            ",
        ))));

        let expected_code: Vec<u8> = vec![
            0b0000000000010000,
            0b1110101010001000,
            0b0000000000010001,
            0b1110111111001000,
            0b0000000000001010,
            0b1110110000010000,
            0b0000000000010010,
            0b1110001100001000,
            0b0000000000010001,
            0b1111110000010000,
            0b1111110111001000,
            0b0000000000010000,
            0b1111000010001000,
            0b0000000000010001,
            0b1111110000010000,
            0b0000000000010010,
            0b1111000111010000,
            0b0000000000001000,
            0b1110001100000011,
        ]
        .into_iter()
        .flat_map(|word: u16| word.to_be_bytes())
        .collect();

        code.iter()
            .zip(&expected_code)
            .for_each(|(byte, expected_byte)| {
                println!("Generated: {byte:08b}\nGExpected: {expected_byte:08b}")
            });
        assert_eq!(code, expected_code);
    }
}
