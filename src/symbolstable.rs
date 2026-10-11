use std::{
    collections::HashMap,
    sync::{
        LazyLock, RwLock, RwLockWriteGuard,
        atomic::{AtomicU16, Ordering},
    },
};

pub type SymbolsTable = HashMap<String, u16>;

pub const INITIAL_MEMORY_INDEX: u16 = 16;

pub static SYMBOLS: RwLock<LazyLock<SymbolsTable>> = RwLock::new(LazyLock::new(|| HashMap::new()));
pub static MEMORY_INDEX: AtomicU16 = AtomicU16::new(INITIAL_MEMORY_INDEX);

pub fn initialize_predefined_symbols() -> () {
    let mut lazy_symbols = obtain_symbols_write();
    let mut symbols = LazyLock::force_mut(&mut lazy_symbols);

    initialize_R_symbols(&mut symbols);

    symbols.insert("SCREEN".to_owned(), 16384);
    symbols.insert("KBD".to_owned(), 24576);
    symbols.insert("SP".to_owned(), 0);
    symbols.insert("LCL".to_owned(), 1);
    symbols.insert("ARG".to_owned(), 2);
    symbols.insert("THIS".to_owned(), 3);
    symbols.insert("THAT".to_owned(), 4);
}

fn initialize_R_symbols(symbols: &mut SymbolsTable) {
    for r in 0..=15 {
        let key = format!("R{r}");
        symbols.insert(key, r);
    }
}

pub fn obtain_symbols_write<'a>() -> RwLockWriteGuard<'a, LazyLock<SymbolsTable>> {
    SYMBOLS
        .write()
        .expect("Failed to obtain write lock on Symbols Table")
}

pub fn get_symbols_table<'a, 'b>(
    lazy_symbols: &'a mut RwLockWriteGuard<'b, LazyLock<SymbolsTable>>,
) -> &'a mut SymbolsTable {
    LazyLock::get_mut(lazy_symbols).expect("Symbols Table must be initialized before retrieval")
}

pub(crate) fn upsert_symbols_table(variable: String) -> u16 {
    let mut lazy_symbols = obtain_symbols_write();
    let symbols = get_symbols_table(&mut lazy_symbols);

    if let Some(var_mem) = symbols.get(&variable) {
        return *var_mem;
    }

    let memory_index = MEMORY_INDEX.fetch_add(1, Ordering::Relaxed);
    symbols.insert(variable, memory_index);
    memory_index
}

pub(crate) fn add_label_to_symbols_table(label_name: String, instruction_offset: u16) -> () {
    let mut lazy_symbols = obtain_symbols_write();
    if let Some(_) = get_symbols_table(&mut lazy_symbols).insert(label_name, instruction_offset) {
        panic!("Duplicate label declaration found");
    }
}
