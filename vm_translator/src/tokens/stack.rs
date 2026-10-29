use crate::{
    tokenizer::TryParse,
    tokens::raw::{RawToken, identifier::Identifier, number::Number},
};

pub enum Stack {
    Push(VirtualMemorySegment, u16),
    Pop(VirtualMemorySegment, u16),
}

pub enum VirtualMemorySegment {
    Local,
    Argument,
    Static,
    Constant,
    This,
    That,
    Temp,
    Pointer,
}

impl Stack {
    fn get_args(iter: &mut impl Iterator<Item = RawToken>) -> (VirtualMemorySegment, u16) {
        (Self::get_segment(iter), Self::get_address(iter))
    }

    fn get_segment(iter: &mut impl Iterator<Item = RawToken>) -> VirtualMemorySegment {
        let expected_msg =
            String::from("Expected a Virtual Memory Segment name after stack operation");
        let RawToken::Identifier(Identifier(segment_name)) =
            iter.next().expect(expected_msg.as_str())
        else {
            panic!("{}", expected_msg);
        };
        use VirtualMemorySegment::*;
        match segment_name.to_lowercase().as_str() {
            "local" => Local,
            "argument" => Argument,
            "static" => Static,
            "constant" => Constant,
            "this" => This,
            "that" => That,
            "temp" => Temp,
            "pointer" => Pointer,
            _ => panic!("{}", expected_msg),
        }
    }

    fn get_address(iter: &mut impl Iterator<Item = RawToken>) -> u16 {
        let expected_msg = String::from("Expected Number token after stack segment");
        let RawToken::Number(Number(i)) = iter.next().expect(expected_msg.as_str()) else {
            panic!("{}", expected_msg);
        };

        i
    }
}

impl TryParse for Stack {
    fn try_parse(value: &str, iter: &mut impl Iterator<Item = RawToken>) -> Option<Self>
    where
        Self: Sized,
    {
        let (segment, i) = Self::get_args(iter);
        match value {
            "push" => Some(Stack::Push(segment, i)),
            "pop" => Some(Stack::Pop(segment, i)),
            _ => None,
        }
    }
}
