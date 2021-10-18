use serde_json;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use crate::utils::{GameBook, OpeningBook};

fn build_interim_book(db: GameBook) -> HashMap<u64, Vec<(u16, u64, bool)>> {
    let mut iterim_book = HashMap::<u64, Vec<(u16, u64, bool)>>::new();

    for (key, count) in db.iter() {
        let (zobrist, mv) = key;
        match iterim_book.get_mut(zobrist) {
            Some(values) => {
                values.push((*mv, count.0, count.1));
            }
            None => {
                iterim_book.insert(*zobrist, vec![(*mv, count.0, count.1)]);
            }
        }
    }

    iterim_book
}

fn find_most_common_move(values: &Vec<(u16, u64, bool)>) -> (u16, u64, bool) {
    let mut mv = 0b0;
    let mut count = 0;
    let mut flag = false;

    for (m, c, f) in values {
        if c > &count {
            count = *c;
            mv = *m;
            flag = *f;
        }
    }

    (mv, count, flag)
}

pub fn build_opening_book(db: GameBook) -> OpeningBook {
    let interim_book = build_interim_book(db);
    let mut opening_book = OpeningBook::new();

    for (zobrist, values) in interim_book.iter() {
        let (mv, count, player) = find_most_common_move(values);
        if count > 30 {
            opening_book.insert(*zobrist, (mv, player));
        }
    }
    opening_book
}

pub fn save_book(path: String, book: &OpeningBook) -> std::io::Result<()> {
    let text = serde_json::to_string(book)?;

    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
