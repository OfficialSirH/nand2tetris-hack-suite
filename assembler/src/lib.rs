pub mod code;
pub mod parser;
pub mod tokenizer;
pub mod tokens;
pub mod utils;

pub const DEFAULT_INPUT_NAME: &'static str = "prog.asm";
pub const DEFAULT_OUTPUT_NAME: &'static str = "prog.hack";

pub fn assemble(assembly: Vec<String>) -> Vec<u8> {
    let mut tokenizer = tokenizer::Tokenizer::new();
    let tokens = tokenizer.tokenize(assembly);
    let parsed_tokens = parser::parse(tokens);
    code::generate(parsed_tokens)
}

/// This function is to convert my beautiful bytes of binary to
/// this goofy file format that the Hack CPU Emulator can understand
/// because for some reason they made the emulator require the
/// instructions to be separated by newlines
pub fn code_to_file_format(code: Vec<u8>) -> Vec<u8> {
    code.chunks_exact(2)
        .map(|word| match word {
            [lbyte, rbyte] => format!("{lbyte:08b}{rbyte:08b}"),
            _ => unreachable!(
                "These are exact chunks and can't ever provide more nor less than 2 elements"
            ),
        })
        .collect::<Vec<String>>()
        .join("\n")
        .into_bytes()
}
