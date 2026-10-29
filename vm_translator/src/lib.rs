mod tokenizer;
mod tokens;

pub const DEFAULT_INPUT_NAME: &'static str = "prog.vm";
pub const DEFAULT_OUTPUT_NAME: &'static str = "prog.asm";

pub fn translate(bytecode: Vec<String>) -> Vec<String> {
    // bytecode.into_iter().map(bytecode_to_assembly)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
