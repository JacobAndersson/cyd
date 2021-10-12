mod book;
mod moves;
mod file;
use std::collections::HashMap;

use pleco::{Board};

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

fn handle_game(game: HashMap<String, String>, db: &mut HashMap<(u64,String), u64>) -> Option<()> {
    let mut board = Board::start_pos();    
    let moves = game.get("moves")?.split(" "); 
    
    for (idx, mut mv) in moves.enumerate() {
        if idx > 30 {
            break;
        }

        if mv.len() == 1 {
            break;
        } else if mv.contains(".") && mv.len() < 4 { //filter move numbers
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

        //println!("{} {} {}", mv, uci_move, valid);
        if valid {
            let hash = board.zobrist();
            let mut count = 1;
            match db.get(&(hash, uci_move.clone())) {
                Some(c) => {
                   count = c + 1;
                },
                None => {}
            }
            db.insert((hash, uci_move.clone()), count);
        } else {
            break;
        }
    }

    Some(())
}


fn main() {
    let lines = file::read_lines("./lichess_elite_2020-06.pgn").unwrap();
    let mut game = HashMap::<String, String>::new();
    let mut db = HashMap::<(u64, String), u64>::new();

    let mut game_idx = 0;

    for (idx, l) in lines.enumerate() {
        let line = l.unwrap();
        if game_idx > 300 { break };

        if idx > 0 && line.len() > 6 && &line[0..6] == "[Event"{
            handle_game(game, &mut db);
            game = HashMap::new();
            game_idx += 1;
        }

        if line != "\n" && line != "" {
            if &line[0..1] == "[" {
                let event = &line[1..(line.len() -1)];
                let (name, value) = split_event(event.to_string()); 
                game.insert(name, value);
            } else {
                match game.get("moves") {
                    Some(moves) => {
                        let new_moves = moves.to_owned() + &line + " ";
                        game.insert("moves".to_string(), new_moves.to_string());
                    },
                    None => {
                        game.insert("moves".to_string(), line + " ");
                    }
                }
            }
        }
    }
    
    let book = book::build_opening_book(db);
    println!("{:?}", book);
    book::save_book("opening_book.json".to_string(), &book);
}
