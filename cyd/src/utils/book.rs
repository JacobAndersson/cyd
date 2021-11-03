use std::collections::HashMap;
use std::fs;
use crate::search::{EntryFlag, TtEntry};
use pleco::BitMove;

fn parse_opening_book() -> Result<HashMap<u64, TtEntry>, std::io::Error> {
    let file = fs::read_to_string("../opening_book.json")?;
    let interim_book: HashMap<u64, (u16, bool)> = serde_json::from_str(&file)?;

    let mut book = HashMap::<u64, TtEntry>::new();

    for (zobrist, (mv, player)) in interim_book {
        let value = match player {
            true => 999.,
            false => -999.,
        };
        let entry = TtEntry {
            mv: BitMove::new(mv),
            depth: 1,
            flag: EntryFlag::Exact,
            value,
        };

        book.insert(zobrist, entry);
    }

    Ok(book)
}

pub fn new_tt_table() -> HashMap<u64, TtEntry> {
    match parse_opening_book() {
        Ok(b) => b,
        Err(_) => HashMap::<u64, TtEntry>::new(),
    }
}
