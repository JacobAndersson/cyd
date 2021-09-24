#[cfg(test)]
mod search_test {
    use crate::utils;
    use pleco::{Board, Player};
    use crate::search::*;

    fn test_position_nega_max(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let board = Board::from_fen(fen).unwrap();
        let (mv, score) = nega_max(board, depth, player);

        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn test_position_alpha_beta(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let mut tt = utils::new_tt_table();
        let board = Board::from_fen(fen).unwrap();
        println!("{}", &board);
        println!("{:?}", &board.turn());
        let (mv, score) = alpha_beta(board, depth, player, -9999.0, 9999.0, &mut tt);
        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn play_x_moves(fen: &str, depth: u8, plies: u8) -> Board {
        let mut board = Board::from_fen(fen).unwrap();
        for _i in 0..plies {
            let mut tt = utils::new_tt_table();

            let (mv, _score) =
                alpha_beta(board.clone(), depth, board.turn(), -9999.0, 9999.0, &mut tt);
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

    #[test]
    fn mate_in_three() {
        let fen = "4r2k/pp4pp/3Q1b2/1n6/4pN2/B3PqP1/5P2/6KR w - - 0 35";
        let depth = 4;
        let correct_move = "f4g6";
        let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
        assert!(find);
    }

    #[test]
    fn find_best_move() {
        let fen = "8/1p3p2/1P2bk2/5p1p/P7/2pB3P/5PP1/5K2 b - - 1 31";
        let depth = 5;
        let correct_move = "e6c4";
        let find = test_position_alpha_beta(fen, depth, Player::Black, correct_move);
        assert!(find);
    }

    #[test] 
    fn find_best_move_2() {
        let fen = "4r1k1/5pp1/1ppR2qp/4P3/1P6/2P5/P3Q1PP/6K1 b - - 1 25";
        let depth = 4;
        let correct_move = "g6d6";
        let find = test_position_alpha_beta(fen, depth, Player::Black, correct_move);
        assert!(find);
    }

    #[test]
    fn find_best_move_3() {
        let fen = "7k/6p1/6Qp/pp1p4/2p5/q1P4P/2P1r1P1/5R1K w - - 0 32";
        let depth = 4;
        let correct_move = "f1f7";
        let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
        assert!(find);
    }

    #[test]
    fn find_best_move_4() {
        let fen = "r4rk1/ppp1nppp/n3b3/1N1p4/2PP4/1Q1BqN2/PP4PP/R4R1K w - - 2 13";
        let depth = 4;
        let correct_move = "d3h7";
        let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
        assert!(find);
    }

    #[test]
    #[ignore]
    fn mate_in_four() {
        let fen = "5q1r/NQ1bkn1p/3pp1p1/p1p3P1/4PP1R/P2PK3/1r6/8 w - - 0 39";
        let depth = 5;
        let board = play_x_moves(fen, depth, 5);
        assert!(board.checkmate());
    }
}
