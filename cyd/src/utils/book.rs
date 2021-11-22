use crate::search::transposition_table::{EntryFlag, TranspositionTable, TtEntry};
use pleco::BitMove;
use std::collections::HashMap;
use std::fs;

fn parse_opening_book() -> Result<TranspositionTable, std::io::Error> {
    let file = fs::read_to_string("../opening_book.json")?;
    let interim_book: HashMap<u64, (u16, bool)> = serde_json::from_str(&file)?;

    let mut book = TranspositionTable::new();

    for (zobrist, (mv, player)) in interim_book {
        let value: i64 = if player { 999 } else { -999 };
        let entry = TtEntry {
            mv: BitMove::new(mv),
            depth: 1,
            flag: EntryFlag::Exact,
            value,
        };

        book.insert_no_refresh(zobrist, entry);
    }
    book.refresh();

    Ok(book)
}

pub fn new_tt_table() -> TranspositionTable {
    match parse_opening_book() {
        Ok(b) => b,
        Err(_) => TranspositionTable::new(),
    }
}
