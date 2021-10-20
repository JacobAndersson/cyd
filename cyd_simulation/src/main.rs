use pleco::{Board, Player};
use cyd::{alpha_beta, new_tt_table, EvalParameters};

use rand_distr::{Distribution, Normal};
use rand::thread_rng;

const DEPTH: u8 = 2;

fn play_game(white_params: EvalParameters, black_params: EvalParameters) -> Option<Player> {
    let mut board = Board::start_pos();

    while !board.checkmate() && board.rule_50() != 50 {
        let mut tt_table = new_tt_table();
        let player = board.turn();
        let params = match player {
            Player::White => white_params,
            Player::Black => black_params,
        };

        let (mv, _) = alpha_beta(board.clone(), DEPTH, player, -9999., 9999., &mut tt_table, true, &Some(params));
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

    for _ in 0..5 {
        match play_game(param1, param2) {
            Some(Player::White) => param1_wins += 1,
            Some(Player::Black) => param2_wins += 1,
            None => {}
        };

        match play_game(param2, param1) {
            Some(Player::White) => param2_wins += 1,
            Some(Player::Black) => param1_wins += 1,
            None => {}
        }
    }

    if param1_wins >= param2_wins { param1 } else { param2 }
}

fn gen_new_parameters(base: &EvalParameters) -> EvalParameters {
    let mut rng = thread_rng();
    let normal = Normal::new(-2., 2.).unwrap();

    EvalParameters {
        psq: base.psq + normal.sample(&mut rng),
        pinned: base.pinned + normal.sample(&mut rng),
        king_safety: base.king_safety + normal.sample(&mut rng)
    }
}


fn main() {
    let mut base = EvalParameters {
       psq: 0.5,
       pinned: 10.,
       king_safety: 2.,
    };
    let mut same_counter = 1;

    while same_counter < 5 {
        let param2 = gen_new_parameters(&base);
        println!("\n{:?}\n{:?}", base, param2);

        let best = battle(base, param2);
        if best == base {
            same_counter += 1;
        } else {
            base = best;
            same_counter = 1;
        }
        println!("{:?}", best);
    }
}
