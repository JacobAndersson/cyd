use pleco::{Board, PieceType, Player};

pub fn piece_values(piece: PieceType) -> f32 {
    return match piece {
        PieceType::P => 100.,
        PieceType::N => 280.,
        PieceType::B => 320.,
        PieceType::R => 479.,
        PieceType::Q => 929.,
        PieceType::K => 60_000.,
        PieceType::None => 0.,
        PieceType::All => 0.,
    };
}

fn count_piece_material(board: &Board, player: Player, piece: PieceType) -> f32 {
    return piece_values(piece) * board.count_piece(player, piece) as f32;
}

fn material_count_side(board: &Board, player: Player) -> f32 {
    let pawns = count_piece_material(board, player, PieceType::P);
    let rook = count_piece_material(board, player, PieceType::R);
    let knight = count_piece_material(board, player, PieceType::N);
    let bishop = count_piece_material(board, player, PieceType::B);
    let queen = count_piece_material(board, player, PieceType::Q);
    pawns + rook + knight + bishop + queen
}

fn material_count(board: &Board) -> f32 {
    material_count_side(board, Player::White) - material_count_side(board, Player::Black)
}

fn piece_square_table(board: &Board) -> f32 {
    let score = board.psq();
    let psq = score.centipawns();
    (psq.0 - psq.1) as f32
}

pub fn eval(board: &Board) -> f32 {
    let material = material_count(board);
    let psq = piece_square_table(board);
    material + 0.1 * psq
}

#[cfg(test)]
mod eval_test {
    use super::*;

    #[test]
    fn material_start_pos() {
        let board = Board::start_pos();
        assert_eq!(0., material_count(&board));
    }

    #[test]
    fn material_single_knight() {
        let board = Board::from_fen("2k5/8/8/8/8/5N2/8/2K5 w - - 0 1").unwrap();
        assert_eq!(280., material_count(&board))
    }

    #[test]
    fn material_black_rook_up() {
        let board = Board::from_fen("2k4r/8/2n5/8/8/5N2/8/2K5 w - - 0 1").unwrap();
        assert_eq!(-479., material_count(&board));
    }

    #[test]
    fn material_white_queen_up() {
        let board = Board::from_fen("2k4r/8/2n5/8/8/1Q3N2/8/2K5 w - - 0 1").unwrap();
        assert_eq!(450., material_count(&board));
    }

    #[test]
    fn psq_start_pos() {
        let board = Board::start_pos();
        assert_eq!(0., piece_square_table(&board))
    }
}
