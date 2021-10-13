use std::collections::HashMap;

use pleco::Board;
use std::fs::File;
use std::io::{self};

use crate::moves;

fn split_event(event: String) -> (String, String) {
    let mut idx = 0;
    for c in event.chars() {
        if c == ' ' {
            let name = event[0..idx].to_string();
            let value = event[idx..].to_string().replace("\"", "");

            return (name, value);
        }
        idx += 1;
    }
    (String::new(), String::new())
}

fn handle_game(
    game: HashMap<String, String>,
    db: &mut HashMap<(u64, String), u64>,
    num_moves: usize,
) -> Option<()> {
    let mut board = Board::start_pos();
    let moves = game.get("moves")?.split(" ");

    for (idx, mut mv) in moves.enumerate() {
        if idx > num_moves {
            break;
        }

        if mv.len() == 1 {
            break;
        } else if mv.contains(".") && mv.len() < 4 {
            //filter move numbers
            continue;
        } else if mv.contains(".") {
            let idx = mv.find(".")?;
            mv = &mv[(idx)..];
            if mv.len() < 2 {
                break;
            }
        } else if mv.contains("-") && (mv.contains("0") || mv.contains("1")) {
            break;
        }

        let uci_move = match moves::algebraic_to_uci_move(mv, &board) {
            Some(x) => x,
            None => {
                println!("RETURNED NONE ON MOVE {:?} {}", mv, board.fen());
                println!("{:?}", game);
                println!("{}", board);
                break;
            }
        };

        let valid = board.apply_uci_move(&uci_move);

        if valid {
            let hash = board.zobrist();
            let mut count = 1;
            match db.get(&(hash, uci_move.clone())) {
                Some(c) => {
                    count = c + 1;
                }
                None => {}
            }
            db.insert((hash, uci_move.clone()), count);
        } else {
            break;
        }
    }

    Some(())
}

pub fn play_through_file(
    file: io::Lines<io::BufReader<File>>,
    db: &mut HashMap<(u64, String), u64>,
    num_moves: usize,
) {
    let mut game = HashMap::<String, String>::new();

    for (idx, l) in file.enumerate() {
        let line = l.unwrap();

        if idx > 0 && line.len() > 6 && &line[0..6] == "[Event" {
            handle_game(game, db, num_moves);
            game = HashMap::new();
        }

        if line != "\n" && line != "" {
            if &line[0..1] == "[" {
                let event = &line[1..(line.len() - 1)];
                let (name, value) = split_event(event.to_string());
                game.insert(name, value);
            } else {
                match game.get("moves") {
                    Some(moves) => {
                        let new_moves = moves.to_owned() + &line + " ";
                        game.insert("moves".to_string(), new_moves.to_string());
                    }
                    None => {
                        game.insert("moves".to_string(), line + " ");
                    }
                }
            }
        }
    }
}
