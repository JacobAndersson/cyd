#[cfg(test)]
mod search_test {
    use crate::search::*;
    use crate::utils;
    use pleco::Board;

    fn test_position_alpha_beta(fen: &str, depth: u8) -> (String, f32) {
        let mut tt = utils::new_tt_table();
        let board = Board::from_fen(fen).unwrap();
        let player = board.turn();
        let (mv, score) = alpha_beta(board, depth, player, -9999.0, 9999.0, &mut tt, true, &None);
        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        (String::from(mv.stringify()), score)
    }

    fn play_x_moves(fen: &str, depth: u8, plies: u8) -> Board {
        let mut board = Board::from_fen(fen).unwrap();
        for _i in 0..plies {
            let mut tt = utils::new_tt_table();

            let (mv, _score) = alpha_beta(
                board.clone(),
                depth,
                board.turn(),
                -9999.0,
                9999.0,
                &mut tt,
                true,
                &None,
            );
            println!("{}", &mv.stringify());
            board.apply_move(mv)
        }
        board
    }

    #[test]
    fn queen_take_white_alpha_beta() {
        let fen = "2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1";
        for depth in 1..3 {
            let (found_move, _) = test_position_alpha_beta(fen, depth);
            assert_eq!(found_move, "c4e6");
        }
    }

    #[test]
    fn queen_take_black_alpha_beta() {
        let fen = "2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1";

        for depth in 1..3 {
            let (found_move, _) = test_position_alpha_beta(fen, depth);
            assert_eq!(found_move, "e7b4");
        }
    }

    #[test]
    fn knight_take_white_alpha_beta() {
        let fen = "2k4r/6pp/8/2p1n3/8/3N4/4PPPP/2K4R w - - 0 1";

        for depth in 1..3 {
            let (found_move, _) = test_position_alpha_beta(fen, depth);
            assert_eq!(found_move, "d3e5");
        }
    }

    #[test]
    fn pin_knight_white_alpha_beta() {
        let fen = "2k4r/6pp/4n3/2p5/8/5B2/4PPPP/2K4R w - - 0 1";

        for depth in 4..5 {
            let (found_move, _) = test_position_alpha_beta(fen, depth);
            assert_eq!(found_move, "f3g4");
        }
    }

    #[test]
    fn mate_in_one_white() {
        let fen = "k7/5R2/6R1/8/8/8/4K3/8 w - - 0 1";

        for depth in 1..4 {
            let (found_move, _) = test_position_alpha_beta(fen, depth);
            assert_eq!(found_move, "g6g8");
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
        let (found_move, _) =
            test_position_alpha_beta("4r2k/pp4pp/3Q1b2/1n6/4pN2/B3PqP1/5P2/6KR w - - 0 35", 4);
        assert_eq!(found_move, "f4g6");
    }

    #[test]
    fn find_best_move() {
        let (found_move, _) =
            test_position_alpha_beta("8/1p3p2/1P2bk2/5p1p/P7/2pB3P/5PP1/5K2 b - - 1 31", 5);
        assert_eq!(found_move, "e6c4");
    }

    #[test]
    fn find_best_move_2() {
        let (found_move, _) =
            test_position_alpha_beta("4r1k1/5pp1/1ppR2qp/4P3/1P6/2P5/P3Q1PP/6K1 b - - 1 25", 4);
        assert_eq!(found_move, "g6d6");
    }

    #[test]
    fn find_best_move_3() {
        let (found_move, _) =
            test_position_alpha_beta("7k/6p1/6Qp/pp1p4/2p5/q1P4P/2P1r1P1/5R1K w - - 0 32", 4);
        assert_eq!(found_move, "f1f7");
    }

    #[test]
    fn find_best_move_4() {
        let (found_move, _) = test_position_alpha_beta(
            "r4rk1/ppp1nppp/n3b3/1N1p4/2PP4/1Q1BqN2/PP4PP/R4R1K w - - 2 13",
            4,
        );
        assert_eq!(found_move, "d3h7");
    }

    #[test]
    fn find_best_move_5() {
        let (found_move, _) = test_position_alpha_beta(
            "6r1/p3kp2/1p2p2p/2rn2q1/3R4/5P2/P1P1B2P/3R1Q1K w - - 12 34",
            4,
        );
        assert_eq!("d4g4", found_move);
    }

    #[test]
    fn find_best_move_6() {
        let fen = "6k1/2R5/2p5/2p1N3/2P1Pp2/1PK1b3/1P2r3/8 b - - 0 43";
        let (found_move, _) = test_position_alpha_beta(fen, 4);
        assert_eq!("e3d4", found_move);

        let fen2 = "6k1/2R5/2p5/2p1N3/2PbPp2/1P1K4/1P2r3/8 b - - 2 44";
        let (found_move2, _) = test_position_alpha_beta(fen2, 4);
        assert_eq!("e2e3", found_move2);
    }

    #[test]
    #[ignore]
    fn mate_in_four() {
        let fen = "r4r1k/1R1R2p1/7p/8/8/3Q1Ppq/P7/6K1 w - - 0 1";
        let (mv, score) = test_position_alpha_beta(fen, 7);
        assert_eq!("d3h7", mv);
        assert!(score > 9000.);
    }

    #[test]
    #[ignore]
    fn mate_in_four_2() {
        let fen = "4r1k1/1pb3pp/2p5/p2p4/P2P4/2B1rBqP/1P3QP1/3K1R2 w - - 4 27";
        let (mv, score) = test_position_alpha_beta(fen, 7);
        assert_eq!("f3d5", mv);
        assert!(score > 9000.);
    }
}
