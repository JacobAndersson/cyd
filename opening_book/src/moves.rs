use pleco::{Board, Player, BitBoard};
use pleco::core::{File, Rank, sq::SQ, PieceType};
use pleco::board::piece_locations::PieceLocations;

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
        _ => Rank::R8 //Should not happen
    }
}

fn parse_rank(mv: &str) -> (i8, Rank) {
    let clean = mv.replace("+", "");
    let rank = &clean[(clean.len() - 1)..];
    let rank_number = rank.parse::<i8>().unwrap();
    (rank_number, get_rank(rank_number))
}

fn get_file(mv: &str) ->  File {
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
        _ => File::H
    }
}


fn get_piece(mv: &str) -> PieceType{
    let lower = mv.to_lowercase();
    
    let mut piece_char: char = ' ';
    for idx in 0..lower.len() {
        if mv[idx..(idx+1)] != lower[idx..(idx+1)] {
            piece_char = mv.chars().nth(idx).unwrap();
        }
    }
    
    match piece_char {
        'N' => PieceType::N,
        'B' =>  PieceType::B,
        'R' => PieceType::R,
        'Q' => PieceType::Q,
        'K' => PieceType::K,
        _ => PieceType::P
    }
}

fn find_start_square(pieces: PieceLocations, piece: PieceType, player: Player, attackers: BitBoard) -> Option<String> {
    for (p_sq, p) in pieces {
        if p.type_of() == piece && p.player_lossy() == player{
            if (attackers & p_sq.to_bb()).count_bits() == 1 {
                return Some(p_sq.to_string());
            } 
        } 
    }
    None
}

pub fn algebraic_to_uci_move(mv: &str, board: &Board) -> Option<String> {
    let player = board.turn();
    let mv = mv.replace("+", "");
    if mv.len() == 5 && mv.contains("O") {
        return match player {
            Player::White => Some("e1c1".to_string()),
            Player::Black => Some("e8c8".to_string())
        }
    } else if mv.contains("#"){
        //checkmate
        let new_move = &mv[0..(mv.len() - 1)];
        let uci_move = algebraic_to_uci_move(new_move, board);
        return uci_move;
    } else if mv.len() == 3 &&  mv.contains("O") {
        return match player {
            Player::White => Some("e1g1".to_string()),
            Player::Black => Some("e8g8".to_string())
        }
    } else if mv.len() == 2 {
        let (num, original_rank) = parse_rank(&mv); 
        let file = get_file(&mv);
        let og_square = SQ::make(file, original_rank);

        let sign = match player {
            Player::White => -1,
            Player::Black => 1
        };

        for i in 1..3 {
            let rank = get_rank(num + i*sign);
            let sq = SQ::make(file, rank);

            let piece = board.piece_at_sq(sq);
            if piece.player_lossy() == player && piece.type_of() == PieceType::P {
                let uci_move = format!("{}{}", sq.to_string(), og_square.to_string());
                return Some(uci_move);
            }
        }
    } else if mv.len() > 2 && mv.chars().nth(mv.len() - 2).unwrap() == '=' {
        let new_move = &mv[0..(mv.len() - 2)];
        let promotion_piece = mv.chars().nth(mv.len() - 1)?.to_lowercase();
        let uci_move = algebraic_to_uci_move(new_move, board)?;
        return Some(format!("{}{}", uci_move, promotion_piece)); 
    } else {
        let rank = get_rank(mv[(mv.len() -1)..].parse::<i8>().unwrap());
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
                },
                None => {
                    let start_file = get_file(&String::from(identifier));
                    attackers = attackers & start_file.bb();
                }
            }
        }

        let start_sq = find_start_square(pieces, piece, player, attackers)?;
        return Some(format!("{}{}", start_sq, sq.to_string()));
    }
    None
}
