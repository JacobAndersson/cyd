use pleco::{BitMove, Board};
use std::collections::HashMap;
use std::fs;

use crate::search;
use crate::search::{EntryFlag, TtEntry};

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

#[allow(dead_code)]
pub fn find_move_fen(fen: String, depth: u8, num_threads: u8) -> (String, f32) {
    match Board::from_fen(&fen) {
        Ok(board) => {
            let (mv, score) =
                search::search_parallel(board.clone(), depth, board.turn(), num_threads);
            (mv.stringify(), score)
        }
        Err(_) => ("".to_string(), 0.),
    }
}

pub fn find_move(moves: String, depth: u8, num_threads: u8) -> (String, f32) {
    let mut board = Board::start_pos();

    let mvs = moves.split(' ');
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), num_threads);
    (mv.stringify(), score)
}

#[allow(dead_code)]
pub fn from_start(depth: u8, n_threads: u8) {
    use std::time::Instant;

    let mut board = Board::start_pos();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), n_threads);
        let end = mv_start.elapsed();
        board.apply_move(mv);

        println!(
            "SCORE: {}, MOVE: {}, player: {}, time: {:?}\n{}\n",
            score,
            &mv,
            board.turn().other_player(),
            end,
            board,
        );
    }
}
