use std::{
    env::Args,
    fs::File,
    io::{BufWriter, Read, Write},
    process::exit,
};

const ASSEMBLE: &'static str = "assemble";
const TRANSLATE: &'static str = "translate";
const COMPILE: &'static str = "compile";

fn main() {
    let args = std::env::args();
    let (input_name, output_name, execute) = get_program_actions(get_args(args));
    let assembly = get_file_contents(input_name);
    let bytes = execute(assembly);
    write_code_to_file(output_name, bytes);
}

fn get_program_actions(
    (program, input_name, output_name): (String, Option<String>, Option<String>),
) -> (String, String, fn(Vec<String>) -> Vec<u8>) {
    let get_program_action =
        |input_default: &str, output_default: &str, executor: fn(Vec<String>) -> Vec<u8>| {
            (
                input_name.unwrap_or(input_default.to_owned()),
                output_name.unwrap_or(output_default.to_owned()),
                executor,
            )
        };
    // TODO: All program functions should output Vec<String> as transformation to their
    // bytes for file writing should only be done after desired steps are done
    match program.as_str() {
        ASSEMBLE => get_program_action(
            assembler::DEFAULT_INPUT_NAME,
            assembler::DEFAULT_OUTPUT_NAME,
            assembler::assemble,
        ),
        TRANSLATE => get_program_action(
            vm_translator::DEFAULT_INPUT_NAME,
            vm_translator::DEFAULT_OUTPUT_NAME,
            vm_translator::translate,
        ),
        COMPILE => get_program_action(
            compiler::DEFAULT_INPUT_NAME,
            compiler::DEFAULT_OUTPUT_NAME,
            compiler::compile,
        ),
        _ => {
            println!("Invalid program");
            exit(1);
        }
    }
}

fn write_code_to_file(output_name: String, bytes: Vec<u8>) {
    let mut file = BufWriter::new(File::create(output_name).expect("Failed to create file"));
    file.write_all(&bytes)
        .expect("Failed to write buffer to file");
}

fn get_file_contents(name: String) -> Vec<String> {
    let mut file = get_file(name);

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("The file's contents must be valid UTF-8");
    let content = content.replace(" ", "");
    let only_asm =
        |line: &str| (!(line.starts_with("//") || line.is_empty())).then_some(line.to_owned());
    let lines = content.split_terminator("\n").filter_map(only_asm);

    lines.collect()
}

fn get_file(name: String) -> File {
    let expectation = format!("No file by the name of {name}");
    let file = File::open(name).expect(expectation.as_str());
    file.lock().unwrap();

    file
}

fn get_args(args: Args) -> (String, Option<String>, Option<String>) {
    let mut args = args.skip(1);
    (args.next().expect("An input for the form of transforming (assemble, translate, compile) needs to be given"), args.next(), args.next())
}
