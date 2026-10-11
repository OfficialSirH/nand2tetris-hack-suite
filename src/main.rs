use std::{
    env::Args,
    fs::File,
    io::{BufWriter, Read, Write},
};

use crate::tokenizer::Tokenizer;

mod code;
mod parser;
mod tokenizer;
mod tokens;
mod utils;

const DEFAULT_INPUT_NAME: &'static str = "prog.asm";
const DEFAULT_OUTPUT_NAME: &'static str = "prog.hack";

fn main() {
    // Read arguments: hack_assembler prog.asm prog.hack
    let args = std::env::args();
    let (input_name, output_name) = get_args(args);
    let asm = get_file_contents(input_name);
    // Send file contents to tokenizer
    let mut tokenizer = Tokenizer::new();
    let tokenized_asm = tokenizer.tokenize(asm);
    // Send tokenized contents to parser
    let parsed_asm = parser::parse(tokenized_asm);
    // Send parsed contents to code generation
    let generated_code = code::generate(parsed_asm);
    // // Write generated code to prog.hack
    write_code_to_file(output_name, generated_code);
}

fn write_code_to_file(output_name: String, generated_code: Vec<u8>) {
    let mut file = BufWriter::new(File::create(output_name).expect("Failed to create file"));
    let hack_format = code_to_file_format(generated_code);
    file.write_all(&hack_format)
        .expect("Failed to write buffer to file");
}

/// This function is to convert my beautiful bytes of binary to
/// this goofy file format that the Hack CPU Emulator can understand
/// because for some reason they made the emulator require the
/// instructions to be separated by newlines
fn code_to_file_format(code: Vec<u8>) -> Vec<u8> {
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

fn get_file_contents(input_name: String) -> Vec<String> {
    let mut file = get_file(input_name);

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let content = content.replace(" ", "");
    let only_asm =
        |line: &str| (!(line.starts_with("//") || line.is_empty())).then_some(line.to_owned());
    let lines = content.split_terminator("\n").filter_map(only_asm);

    lines.collect()
}

fn get_file(input_name: String) -> File {
    let expectation = format!("No file by the name of {input_name}");
    let file = File::open(input_name).expect(expectation.as_str());
    file.lock().unwrap();

    file
}

fn get_args(args: Args) -> (String, String) {
    let mut args = args.skip(1);
    let mut get_arg = |fallback: &str| args.next().unwrap_or(fallback.to_owned());
    (get_arg(DEFAULT_INPUT_NAME), get_arg(DEFAULT_OUTPUT_NAME))
}
