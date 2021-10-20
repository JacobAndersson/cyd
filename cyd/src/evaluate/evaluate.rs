use pleco::{Board, PieceType, Player};
use pleco::helper::Helper;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EvalParameters {
    pub psq: f32,
    pub pinned: f32,
    pub king_safety: f32,
}

impl Default for EvalParameters {
    fn default() -> Self {
        EvalParameters {
            psq: 0.5,
            pinned: 10.,
            king_safety: 2.
        }
    }
}

pub fn piece_values(piece: PieceType) -> f32 {
    match piece {
        PieceType::P => 100.,
        PieceType::N => 280.,
        PieceType::B => 320.,
        PieceType::R => 479.,
        PieceType::Q => 929.,
        PieceType::K => 60_000.,
        PieceType::None => 0.,
        PieceType::All => 0.,
    }
}

fn count_piece_material(board: &Board, player: Player, piece: PieceType) -> f32 {
    piece_values(piece) * board.count_piece(player, piece) as f32
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

fn pinned_pieces(board: &Board) -> f32 {
    let wp = board.pieces_pinned(Player::White).count_bits() as f32;
    let bp = board.pieces_pinned(Player::Black).count_bits() as f32;
    bp - wp
}

fn king_safety(board: &Board, player: Player) -> f32 {
    let king = board.king_sq(player);  

    let helper = Helper::new();

    let around = helper.ring_distance(king, 1);
    let occupied = board.get_occupied_player(player);

    (around & occupied).count_bits() as f32
}

fn eval_raw(board: &Board, params: &EvalParameters ) -> f32 {
    if board.checkmate() {
        let turn: f32 = match &board.turn() {
            Player::White => 1.0,
            Player::Black => -1.0,
        };
        return -turn * (9999.0 - board.ply() as f32);
    }

    let material = material_count(board);
    let psq = params.psq * piece_square_table(board);
    let pinned = params.pinned * pinned_pieces(board);

    let king_safety_white = king_safety(board, Player::White);
    let king_safety_black = king_safety(board, Player::Black);
    let k_safety = params.king_safety*(king_safety_white - king_safety_black);

    material + psq + pinned + k_safety 
}

pub fn eval(board: &Board, params: &Option<EvalParameters>) -> f32 {
    match params {
        Some(p) => eval_raw(board, p),
        None => eval_raw(board, &EvalParameters::default())
    }
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

    #[test]
    fn checkmate_white() {
        let board = Board::from_fen("4R2k/6pp/8/2p5/6n1/5B2/4PPPP/2K4R b - - 0 1").unwrap();
        assert_eq!(9999.0, eval(&board, &None));
    }

    #[test]
    fn checkmate_black() {
        let board = Board::from_fen("3k4/8/8/8/8/8/P7/K1q5 w - - 0 1").unwrap();
        assert_eq!(-9999.0, eval(&board, &None));
    }


    #[test]
    fn test_pinned_pieces() {
        let b1 = Board::from_fen("2k5/3p4/8/5B2/8/8/8/2K5 w - - 0 1").unwrap();
        let p1 = pinned_pieces(&b1);
        assert_eq!(p1, 1.0);
        let b2 = Board::from_fen("2k5/3p4/2r5/5B2/8/8/2P5/2K5 w - - 0 1").unwrap();
        let p2 = pinned_pieces(&b2);
        assert_eq!(p2, 0.0);

        let b3 = Board::from_fen("3k4/8/8/3r4/8/8/3P4/3K4 w - - 0 1").unwrap();
        let p3 = pinned_pieces(&b3);
        assert_eq!(p3, -1.0);

        let b4 = Board::from_fen("3k4/8/8/3r4/8/1b6/2PP4/3KN2q w - - 0 1").unwrap();
        let p4 = pinned_pieces(&b4);
        assert_eq!(p4, -3.0);
    }
}
