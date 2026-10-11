use std::collections::HashMap;

use crate::tokens::Instruction;

pub type SymbolsTable = HashMap<String, u16>;

pub const INITIAL_MEMORY_INDEX: u16 = 16;

pub struct Tokenizer {
    symbols_table: SymbolsTable,
    memory_index: u16,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            symbols_table: Self::initialize_predefined_symbols(),
            memory_index: INITIAL_MEMORY_INDEX,
        }
    }

    fn initialize_predefined_symbols() -> SymbolsTable {
        let mut symbols: SymbolsTable = HashMap::new();
        Self::initialize_r_symbols(&mut symbols);

        symbols.insert("SCREEN".to_owned(), 16384);
        symbols.insert("KBD".to_owned(), 24576);
        symbols.insert("SP".to_owned(), 0);
        symbols.insert("LCL".to_owned(), 1);
        symbols.insert("ARG".to_owned(), 2);
        symbols.insert("THIS".to_owned(), 3);
        symbols.insert("THAT".to_owned(), 4);

        symbols
    }

    fn initialize_r_symbols(symbols: &mut SymbolsTable) {
        for r in 0..=15 {
            let key = format!("R{r}");
            symbols.insert(key, r);
        }
    }

    pub fn upsert_symbols_table(&mut self, variable: String) -> u16 {
        if let Some(var_mem) = self.symbols_table.get(&variable) {
            return *var_mem;
        }

        let memory_index = self.memory_index;
        self.memory_index += 1;
        self.symbols_table.insert(variable, memory_index);
        memory_index
    }

    /// Inserts label into the symbols table with provided instruction offset
    ///
    /// # Panics
    ///
    /// The function panics if the label already exists in the symbols table
    pub fn insert_label(&mut self, label_name: String, instruction_offset: u16) -> () {
        if let Some(_) = self.symbols_table.insert(label_name, instruction_offset) {
            panic!("Duplicate label declaration found");
        }
    }

    pub fn tokenize(&mut self, assembly: Vec<String>) -> Vec<Instruction> {
        let mut instruction_offset: u16 = 0;

        let tokenize_instruction = |instruction: String| {
            (!self.parse_label(&instruction, instruction_offset)).then(|| {
                instruction_offset += 1;
                Instruction::tokenize(self, instruction)
            })
        };

        assembly
            .into_iter()
            .filter_map(tokenize_instruction)
            .collect()
    }

    fn parse_label(&mut self, instruction: &str, instruction_offset: u16) -> bool {
        let mut chars = instruction.chars();
        let mut label_name = String::new();
        if let Some(char) = chars.next()
            && char != '('
        {
            return false;
        }

        while let Some(char) = chars.next() {
            if char == ')' {
                break;
            }

            if !char.is_alphabetic() {
                panic!("Invalid characters in label");
            }

            label_name.push(char);
        }

        if chars.count() > 0 {
            panic!("Unexpected characters at end of label");
        }

        self.insert_label(label_name, instruction_offset);
        return true;
    }
}

pub trait Tokenize<T> {
    fn tokenize(ctx: &mut Tokenizer, value: T) -> Self;
}

pub trait MaybeTokenize<T>
where
    Self: Sized,
{
    fn maybe_tokenize(ctx: &mut Tokenizer, value: T) -> Option<Self>;
}

#[cfg(test)]
mod parse_tests {
    use crate::{
        tokenizer::Tokenizer,
        tokens::{
            AInstruction, CInstruction, CToken, Delimiter, Function, Instruction, Jump, Value,
        },
        utils::multiline_to_clean_vec,
    };

    fn num_to_atoken(value: &u16) -> Instruction {
        Instruction::A(AInstruction(*value))
    }

    fn ctoken_to_cinstruction(value: Vec<CToken>) -> Instruction {
        Instruction::C(CInstruction(value))
    }

    #[test]
    fn a_instructions() {
        let mut tokenizer = Tokenizer::new();

        let instructions = multiline_to_clean_vec(
            "@10
            @var
            @27
            @var2
            @var
            @var3",
        );
        let instruction_tokens = vec![10, 16, 27, 17, 16, 18]
            .iter()
            .map(num_to_atoken)
            .collect::<Vec<Instruction>>();
        assert_eq!(tokenizer.tokenize(instructions), instruction_tokens);
    }

    #[test]
    fn c_instructions() {
        let mut tokenizer = Tokenizer::new();

        let instructions = multiline_to_clean_vec(
            "A
             1
             D+M
             A=D
             0;JMP",
        );
        let instruction_tokens = vec![
            vec![CToken::Value(Value::A)],
            vec![CToken::Value(Value::One)],
            vec![
                CToken::Value(Value::D),
                CToken::Function(Function::Add),
                CToken::Value(Value::M),
            ],
            vec![
                CToken::Value(Value::A),
                CToken::Delimiter(Delimiter::Assign),
                CToken::Value(Value::D),
            ],
            vec![
                CToken::Value(Value::Zero),
                CToken::Delimiter(Delimiter::Semicolon),
                CToken::Jump(Jump::JMP),
            ],
        ]
        .into_iter()
        .map(ctoken_to_cinstruction)
        .collect::<Vec<Instruction>>();
        assert_eq!(tokenizer.tokenize(instructions), instruction_tokens);
    }
}
