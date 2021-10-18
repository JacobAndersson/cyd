use pleco::board::piece_locations::PieceLocations;
use pleco::core::{sq::SQ, File, PieceType, Rank};
use pleco::{BitBoard, BitMove, Board, Player};

fn get_rank(r: i8) -> Rank {
    match r {
        1 => Rank::R1,
        2 => Rank::R2,
        3 => Rank::R3,
        4 => Rank::R4,
        5 => Rank::R5,
        6 => Rank::R6,
        7 => Rank::R7,
        8 => Rank::R8,
        _ => Rank::R8, //Should not happen
    }
}

fn parse_rank(mv: &str) -> (i8, Rank) {
    let clean = mv.replace("+", "");
    let rank = &clean[(clean.len() - 1)..];
    let rank_number = rank.parse::<i8>().unwrap();
    (rank_number, get_rank(rank_number))
}

fn get_file(mv: &str) -> File {
    let letter = &mv[0..1];
    match letter {
        "a" => File::A,
        "b" => File::B,
        "c" => File::C,
        "d" => File::D,
        "e" => File::E,
        "f" => File::F,
        "g" => File::G,
        "h" => File::H,
        _ => File::H,
    }
}

fn get_piece(mv: &str) -> PieceType {
    let lower = mv.to_lowercase();

    let mut piece_char: char = ' ';
    for idx in 0..lower.len() {
        if mv[idx..(idx + 1)] != lower[idx..(idx + 1)] {
            piece_char = mv.chars().nth(idx).unwrap();
        }
    }

    match piece_char {
        'N' => PieceType::N,
        'B' => PieceType::B,
        'R' => PieceType::R,
        'Q' => PieceType::Q,
        'K' => PieceType::K,
        _ => PieceType::P,
    }
}

fn find_start_square(
    pieces: PieceLocations,
    piece: PieceType,
    player: Player,
    attackers: BitBoard,
) -> Option<SQ> {
    for (p_sq, p) in pieces {
        if p.type_of() == piece && p.player_lossy() == player {
            if (attackers & p_sq.to_bb()).count_bits() == 1 {
                return Some(p_sq);
            }
        }
    }
    None
}

pub fn algebraic_to_uci_move(mv: &str, board: &Board) -> Option<BitMove> {
    let player = board.turn();
    let mv = mv.replace("+", "");
    if mv.len() == 5 && mv.contains("O") {
        let rank = match player {
            Player::White => Rank::R1,
            Player::Black => Rank::R8,
        };

        let src = SQ::make(File::E, rank);
        let dst = SQ::make(File::C, rank);
        let bit_mv = BitMove::make(0b0011, src, dst);

        return Some(bit_mv);
    } else if mv.len() == 3 && mv.contains("O") {
        let rank = match player {
            Player::White => Rank::R1,
            Player::Black => Rank::R8,
        };

        let src = SQ::make(File::E, rank);
        let dst = SQ::make(File::G, rank);
        let bit_mv = BitMove::make(0b0001, src, dst);

        return Some(bit_mv);
    } else if mv.contains("#") {
        //checkmate
        let new_move = &mv[0..(mv.len() - 1)];
        let uci_move = algebraic_to_uci_move(new_move, board);
        return uci_move;
    } else if mv.len() == 2 {
        //Pawn push
        let (num, original_rank) = parse_rank(&mv);
        let file = get_file(&mv);
        let dst = SQ::make(file, original_rank);

        let sign = match player {
            Player::White => -1,
            Player::Black => 1,
        };

        for i in 1..3 {
            let rank = get_rank(num + i * sign);
            let sq = SQ::make(file, rank);

            let piece = board.piece_at_sq(sq);
            if piece.player_lossy() == player && piece.type_of() == PieceType::P {
                let bt_move = BitMove::make_pawn_push(sq, dst);
                return Some(bt_move);
            }
        }
    } else if mv.len() > 2 && mv.chars().nth(mv.len() - 2).unwrap() == '=' {
        let new_move = &mv[0..(mv.len() - 2)];
        let promotion_piece = mv.chars().nth(mv.len() - 1)?;
        let base_mv = algebraic_to_uci_move(new_move, board)?;

        let mut bit = match promotion_piece {
            'N' => 0b1000,
            'B' => 0b1001,
            'R' => 0b1010,
            'Q' => 0b1011,
            _ => 0b1011,
        };

        if mv.contains("x") {
            bit = bit | 0b0100;
        }

        let dst = base_mv.get_dest();
        let src = base_mv.get_src();
        let bt_mv = BitMove::make(bit, src, dst);

        return Some(bt_mv);
    } else {
        let rank = get_rank(mv[(mv.len() - 1)..].parse::<i8>().unwrap());
        let file = get_file(&mv[(mv.len() - 2)..(mv.len() - 1)]);
        let piece = get_piece(&mv);

        let sq = SQ::make(file, rank);
        let occupied_map = board.get_occupied_player(player);
        let mut attackers = board.attackers_to(sq, occupied_map);
        let pieces = board.get_piece_locations();

        if mv.len() == 4 && !mv.contains("x") {
            let identifier = mv.chars().nth(1)?;

            match identifier.to_digit(10) {
                Some(r) => {
                    let start_rank = get_rank(r as i8);
                    attackers = attackers & start_rank.bb();
                }
                None => {
                    let start_file = get_file(&String::from(identifier));
                    attackers = attackers & start_file.bb();
                }
            }
        }

        let bit = if mv.contains("x") { 0b0100 } else { 0b0000 };

        let start_sq = find_start_square(pieces, piece, player, attackers)?;
        let bt_mv = BitMove::make(bit, start_sq, sq);
        return Some(bt_mv);
    }
    None
}
