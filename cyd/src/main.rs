mod evaluate;
mod search;

use pleco::{Board, Player};
use dashmap::DashMap;

use std::time::Instant;
use std::collections::HashMap;

use std::sync::{Arc, RwLock};

fn main() {
    //dashmap
    let board = Board::start_pos();
    let depth = 7;
    let color = Player::White;
    let t0 = Instant::now();
    let mut tt = Arc::new(DashMap::<u64, search::TtEntry>::new());
    //search::search_parallel(board, depth, color, 3);
    search::alpha_beta(board, depth, color, -9999.0, 9999.0, &mut tt);
    println!("{:?}", t0.elapsed());

    /*
    let mut tt = Arc::new(Mutex::new(HashMap::<u64, String>::new()));
    let mut threads = vec![];

    for i in 0..10000 {
        let table = tt.clone();
        threads.push(thread::spawn(move || {
            let t0 = Instant::now();
            let mut access = table.lock().unwrap();
            access.insert(i, "TESTING".to_string());
            println!("{:?}", t0.elapsed());
        }));
    }
    
    for t in threads {
        t.join();
    }

    //println!("{:?}", tt);
    */

    /*
    let mut board = Board::start_pos();
    const SEARCH_DEPTH: u8 = 5;

    let t0 = Instant::now();
    while !board.checkmate() && board.rule_50() != 50 {
        let mut tt: HashMap<u64, search::TtEntry> = HashMap::new();
        let mv_start = Instant::now();
        let (mv, score) = search::alpha_beta(
            board.clone(),
            SEARCH_DEPTH,
            board.turn(),
            -9999.,
            9999.,
            &mut tt,
        );
        let end = mv_start.elapsed();
        board.apply_move(mv);
        println!(
            "SCORE: {}, MOVE: {}, player: {}, time: {:?}",
            score,
            &mv,
            board.turn().other_player(),
            end
        );
        println!("{}", board);
    }

    println!("GAME TOOK: {:?}", t0.elapsed())
    */
}
