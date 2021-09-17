use crate::evaluate::eval;
use pleco::{BitMove, Board, Player};

fn color_value(player: Player) -> f32 {
    return match player {
        Player::White => 1.,
        Player::Black => -1.,
    };
}

#[allow(dead_code)] //For benchmarks
pub fn nega_max(mut board: Board, depth: u8, color: Player) -> (BitMove, f32) {
    if depth == 0 {
        return (BitMove::null(), color_value(color) * eval(&board));
    }

    let mut max = -999999.;
    let mut best_move = BitMove::null();

    for mv in board.generate_moves() {
        board.apply_move(mv);
        let (_, mut score) = nega_max(board.shallow_clone(), depth - 1, color.other_player());
        score = -score;

        if score > max {
            max = score;
            best_move = mv;
        }
        board.undo_move();
    }

    (best_move, max)
}

fn quiesce(mut board: Board, depth: u8, color: Player, mut alpha: f32, beta: f32) -> f32 {
    let standpat = color_value(color) * eval(&board);
    if depth == 0 {
        return standpat;
    } else if standpat >= beta {
        return beta;
    } else if alpha < standpat {
        alpha = standpat;
    }

    let moves = board.generate_moves();
    for mv in moves {
        //Should be possible to only generate capturing moves. Problem with check
        if !board.is_capture(mv) {
            continue;
        }

        board.apply_move(mv);
        let score = -quiesce(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
        );
        board.undo_move();

        if score >= beta {
            return beta;
        } else if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}

pub fn alpha_beta(
    mut board: Board,
    depth: u8,
    color: Player,
    mut alpha: f32,
    beta: f32,
) -> (BitMove, f32) {
    let moves = board.generate_moves();

    if depth == 0 || board.checkmate() || moves.is_empty() {
        return (BitMove::null(), quiesce(board, 10, color, alpha, beta));
    }

    let mut best_move = BitMove::null();
    for mv in moves {
        board.apply_move(mv);
        let (_, mut score) = alpha_beta(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
        );
        score = -score;

        if score >= beta {
            return (mv, beta);
        } else if score > alpha {
            alpha = score;
            best_move = mv;
        }
        board.undo_move();
    }

    (best_move, alpha)
}

#[cfg(test)]
mod search_test {
    use super::*;

    fn test_position_nega_max(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let board = Board::from_fen(fen).unwrap();
        let (mv, score) = nega_max(board, depth, player);

        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn test_position_alpha_beta(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let board = Board::from_fen(fen).unwrap();
        let (mv, score) = alpha_beta(board, depth, player, -9999.0, 9999.0);
        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn play_x_moves(fen: &str, depth: u8, plies: u8) -> Board {
        let mut board = Board::from_fen(fen).unwrap();
        for _i in 0..plies {
            let (mv, _score) = alpha_beta(board.clone(), depth, board.turn(), -9999.0, 9999.0);
            board.apply_move(mv)
        }
        board
    }

    #[test]
    fn queen_take_white() {
        let fen = "2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1";
        let correct_move = "c4e6";
        for depth in 1..3 {
            let found = test_position_nega_max(&fen, depth, Player::White, &correct_move);
            assert!(found);
        }
    }

    #[test]
    fn queen_take_black() {
        let fen = "2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1";
        let correct_move = "e7b4";
        for depth in 1..3 {
            let found = test_position_nega_max(&fen, depth, Player::Black, &correct_move);
            assert!(found);
        }
    }

    #[test]
    fn queen_take_white_alpha_beta() {
        let fen = "2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1";
        let correct_move = "c4e6";
        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn queen_take_black_alpha_beta() {
        let fen = "2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1";
        let correct_move = "e7b4";
        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::Black, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn knight_take_white_alpha_beta() {
        let fen = "2k4r/6pp/8/2p1n3/8/3N4/4PPPP/2K4R w - - 0 1";
        let correct_move = "d3e5";

        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn pin_knight_white_alpha_beta() {
        let fen = "2k4r/6pp/4n3/2p5/8/5B2/4PPPP/2K4R w - - 0 1";
        let correct_move = "f3g4";

        for depth in 4..5 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_one_white() {
        let fen = "k7/5R2/6R1/8/8/8/4K3/8 w - - 0 1";
        let correct_move = "g6g8";

        for depth in 1..4 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_one_black() {
        let fen = "1k6/8/8/8/8/3n4/6PR/6RK b Q - 0 1";
        let correct_move = "d3f2";
        for depth in 1..4 {
            let find = test_position_nega_max(fen, depth, Player::Black, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_two_white() {
        let fen = "k7/4R3/8/8/8/4R3/8/3K4 w - - 0 1";
        for depth in 4..6 {
            let board = play_x_moves(fen, depth, 3);
            assert!(board.checkmate());
        }
    }

    #[test]
    fn mate_in_two_2() {
        let fen = "k7/4R3/2p5/p7/1p6/2P1R2P/1P4P1/3K4 w - - 0 1";
        for depth in 4..6 {
            let board = play_x_moves(fen, depth, 3);
            assert!(board.checkmate());
        }
    }
}