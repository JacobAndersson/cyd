use serde_json;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

fn build_interim_book(db: HashMap<(u64, String), u64>) -> HashMap<u64, Vec<(String, u64)>> {
    let mut iterim_book = HashMap::<u64, Vec<(String, u64)>>::new();

    for (key, count) in db.iter() {
        let (zobrist, mv) = key;
        match iterim_book.get_mut(zobrist) {
            Some(values) => {
                values.push((mv.to_string(), *count));
            }
            None => {
                iterim_book.insert(*zobrist, vec![(mv.to_string(), *count)]);
            }
        }
    }

    iterim_book
}

fn find_most_common_move(values: &Vec<(String, u64)>) -> (String, u64) {
    let mut mv = " ".to_string();
    let mut count = 0;

    for (m, c) in values {
        if c > &count {
            count = *c;
            mv = m.to_string();
        }
    }

    (mv, count)
}

pub fn build_opening_book(db: HashMap<(u64, String), u64>) -> HashMap<u64, String> {
    let interim_book = build_interim_book(db);
    let mut opening_book = HashMap::<u64, String>::new();

    for (zobrist, values) in interim_book.iter() {
        let (mv, count) = find_most_common_move(values);
        if count > 10 {
            opening_book.insert(*zobrist, mv);
        }
    }
    opening_book
}

pub fn save_book(path: String, book: &HashMap<u64, String>) -> std::io::Result<()> {
    let text = serde_json::to_string(book)?;

    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
