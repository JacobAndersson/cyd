mod book;
mod file;
mod game;
mod moves;

use std::collections::HashMap;

fn main() {
    let lines = file::read_lines("./lichess_elite_2020-06.pgn").unwrap();
    let mut db = HashMap::<(u64, String), u64>::new();

    game::play_through_file(lines, &mut db, 10);
    let book = book::build_opening_book(db);
    println!("{:?}", book);
    book::save_book("opening_book.json".to_string(), &book).unwrap();
}
