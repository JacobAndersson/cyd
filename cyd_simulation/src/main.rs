use pleco::{Board, Player};
use cyd::{alpha_beta, new_tt_table, EvalParameters};

use rand_distr::{Distribution, Normal};
use rand::thread_rng;

const DEPTH: u8 = 2;

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

    let moves = ["c2d2", "e2e4", "c2c4", "g1f3"];

    for mv in moves {
        match play_game(param1, param2, &mv) {
            Some(Player::White) => param1_wins += 1,
            Some(Player::Black) => param2_wins += 1,
            None => {}
        };

        match play_game(param2, param1, &mv) {
            Some(Player::White) => param2_wins += 1,
            Some(Player::Black) => param1_wins += 1,
            None => {}
        }
    }

    if param1_wins >= param2_wins { param1 } else { param2 }
}

fn gen_new_parameters(base: &EvalParameters, rands: &ParameterRands) -> EvalParameters {
    let mut rng = thread_rng();

    EvalParameters {
        psq: base.psq + rands.psq.sample(&mut rng),
        pinned: base.pinned + rands.pinned.sample(&mut rng),
        king_safety: base.king_safety + rands.king_safety.sample(&mut rng)
    }
}


fn main() {
    let rands = ParameterRands {
        psq: Normal::new(0., 0.5).unwrap(),
        pinned: Normal::new(0., 2.).unwrap(),
        king_safety: Normal::new(0., 1.5).unwrap()
    };


    let mut base = EvalParameters {
       psq: 0.5,
       pinned: 10.,
       king_safety: 2.,
    };

    let mut same_counter = 1;

    while same_counter < 20 {
        let param2 = gen_new_parameters(&base, &rands);
        println!("\n{:?}\n{:?}", base, param2);

        let best = battle(base, param2);
        if best == base {
            same_counter += 1;
        } else {
            base = best;
            same_counter = 1;
        }
    }
}
