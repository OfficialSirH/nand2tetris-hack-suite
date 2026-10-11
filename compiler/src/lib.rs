pub const DEFAULT_INPUT_NAME: &'static str = "prog.jack";
// TODO: rename file ending once I've read into the VM chapters
pub const DEFAULT_OUTPUT_NAME: &'static str = "prog.bytecode";

pub fn compile(source_code: Vec<String>) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
