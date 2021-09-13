use crate::evaluate::eval;

use pleco::{BitMove, Board, Player};

fn color_value(player: Player) -> f32 {
    return match player {
        Player::White => 1.,
        Player::Black => -1.,
    };
}

pub fn search(mut board: Board, depth: u8, color: Player) -> (BitMove, f32) {
    if depth == 0 {
        return (BitMove::null(), color_value(color) * eval(&board));
    }

    let mut max = -999999.;
    let mut best_move = BitMove::null();

    for mv in board.generate_moves() {
        board.apply_move(mv);
        let (_, mut score) = search(board.shallow_clone(), depth - 1, color.other_player());
        score = -score;

        if score > max {
            max = score;
            best_move = mv;
        }
        board.undo_move();
    }

    (best_move, max)
}

#[cfg(test)]
mod search_test {
    use super::*;

    #[test]
    fn queen_take_white() {
        for depth in 1..3 {
            let board = Board::from_fen("2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1").unwrap();
            let (mv, score) = search(board, depth, Player::White);
            println!("depth: {}, move: {}, score: {}", depth, mv, score);
            assert_eq!("c4e6", mv.stringify());
        }
    }

    #[test]
    fn queen_take_black() {
        for depth in 1..3 {
            let board = Board::from_fen("2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1").unwrap();
            let (mv, score) = search(board, depth, Player::Black);
            println!("depth: {}, move: {}, score: {}", depth, mv, score);
            assert_eq!("e7b4", mv.stringify());
        }
    }
}
