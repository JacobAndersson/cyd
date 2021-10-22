use std::thread;
use std::time::Instant;

use pleco::{Board, Player};
use cyd::{alpha_beta, new_tt_table, EvalParameters};

use rand_distr::{Distribution, Normal};
use rand::thread_rng;

const DEPTH: u8 = 3;

struct ParameterRands {
    psq: Normal<f32>,
    pinned: Normal<f32>,
    king_safety: Normal<f32>
}

fn play_game(white_params: EvalParameters, black_params: EvalParameters, mv: &str) -> Option<Player> {
    let mut board = Board::start_pos();
    board.apply_uci_move(mv);

    while !board.checkmate() && board.rule_50() != 50 {
        let mut tt_table = new_tt_table();
        let player = board.turn();
        let params = match player {
            Player::White => white_params,
            Player::Black => black_params,
        };

        let (mv, _) = alpha_beta(board.clone(), DEPTH, player, -9999., 9999., &mut tt_table, true, &Some(params));
        if mv.to_string() == "a1a1" {
            break;
        }
        board.apply_move(mv);
    }

    if board.checkmate() {
        Some(board.turn())
    }  else {
        None
    }
}

fn battle(param1: EvalParameters, param2: EvalParameters) -> EvalParameters {
    let mut param1_wins = 0;
    let mut param2_wins = 0;

    let moves = ["d2d4", "e2e4", "c2c4", "g1f3"];
    let t0 = Instant::now();

    let mut threads = vec![];

    for mv in moves {
        threads.push(thread::spawn(move || {
            let mut p1 = 0;
            let mut p2 = 0;
            match play_game(param1, param2, &mv) {
                Some(Player::White) => p1 += 1,
                Some(Player::Black) => p2 += 1,
                None => {}
            };

            match play_game(param2, param1, &mv) {
                Some(Player::White) => p2 += 1,
                Some(Player::Black) => p1 += 1,
                None => {}
            }
            (p1, p2)
        }));
    }


    for t in threads {
        let (p1, p2) = t.join().unwrap();
        param1_wins += p1;
        param2_wins += p2;
    }

    println!("Battle took: {:?}", t0.elapsed());

    if param1_wins >= param2_wins { param1 } else { param2 }
}


fn gen_new(param: f32, dist: Normal<f32>, temp: f32) -> f32 {
    let mut rng = thread_rng();
    0_f32.max(param + dist.sample(&mut rng) * temp)
}

fn gen_new_parameters(base: &EvalParameters, rands: &ParameterRands, i: f32, max_i: f32) -> EvalParameters {
    
    
    let temp = 1. - i/max_i;

    EvalParameters {
        psq: gen_new(base.psq, rands.psq, temp),
        pinned: gen_new(base.pinned, rands.pinned, temp),
        king_safety: gen_new(base.king_safety, rands.king_safety, temp)
    }
}


fn main() {
    let rands = ParameterRands {
        psq: Normal::new(0.5, 0.5).unwrap(),
        pinned: Normal::new(10., 10.).unwrap(),
        king_safety: Normal::new(10., 10.).unwrap()
    };


    let mut best = EvalParameters {
       psq: 0.5,
       pinned: 10.,
       king_safety: 2.,
    };

    let mut same_counter = 1;
    let streak = 30;

    while same_counter < streak {
        let param2 = gen_new_parameters(&best, &rands, same_counter as f32, streak as f32);
        println!("\n{:?}\n{:?}", best, param2);

        let winner = battle(best, param2);
        if winner == best {
            same_counter += 1;
        } else {
            best = winner;
            same_counter = 1;
        }
    }

    println!("FINAL WINNER {:?}", best);
}
