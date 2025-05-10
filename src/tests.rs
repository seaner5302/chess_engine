#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::Bitboard;
    use crate::moves::*;

    fn print_results(fen: &str, square: u64, test_name: &str, expected_moves: &[u64], is_white: bool, verbose: bool) -> bool {
        let board = Bitboard::from_fen(fen);
        let moves = get_pawn_moves(&board, square, is_white);
        let board_str = board.to_string();
        
        if verbose {
            println!("Test Name: {}", test_name);
            println!("{}", board_str);
            println!("Getting moves for {} pawn on {}", if is_white { "white" } else { "black" }, square_to_algebraic(square));
            print!("[");
            
            for (i, &mv) in moves.iter().enumerate() {
                print!("{} {}", mv, square_to_algebraic(mv));
                if i != moves.len() - 1 {
                    print!(", ");
                }
            }
            println!("]");
            
            let valid = if expected_moves.len() == 1 && expected_moves[0] == 0 {
                moves.is_empty()
            } else {
                moves == expected_moves
            };
            
            if valid {
                println!("VALID");
            } else {
                println!("INVALID");
                print!("Expected moves: [");
                for (i, &mv) in expected_moves.iter().enumerate() {
                    if mv != 0 {
                        print!("{} {}", mv, square_to_algebraic(mv));
                        if i != expected_moves.len() - 1 {
                            print!(", ");
                        }
                    }
                }
                println!("]");
            }
            println!("------------------------------------------------------------");
            valid
        } else {
            if expected_moves.len() == 1 && expected_moves[0] == 0 {
                moves.is_empty()
            } else {
                moves == expected_moves
            }
        }
    }

    fn run_all_tests(test_names: &[&str], squares: &[u64], board_states: &[&str], expected_moves: &[&[u64]], is_white: bool, verbose: bool) {
        let mut validity = vec![true; test_names.len()];
        let mut successes = 0;
        let mut failures = 0;
        
        println!("\nRunning {} Pawn Move Tests", if is_white { "White" } else { "Black" });
        println!("------------------------------------------------------------");
        
        for i in 0..test_names.len() {
            let state_name = format!("Board state: {}", test_names[i]);
            validity[i] = print_results(board_states[i], squares[i], &state_name, expected_moves[i], is_white, true);
        }
        
        println!("\nTest Results Summary");
        println!("------------------------------------------------------------");
        for (i, (name, valid)) in test_names.iter().zip(validity.iter()).enumerate() {
            let needs_space = (i + 1) / 10 == 0;
            let sp = if needs_space { " " } else { "" };
            
            if *valid {
                println!("| Test Number: {} {} | Name: {} | PASSED |", sp, i + 1, name);
                successes += 1;
            } else {
                println!("| Test Number: {} {} | Name: {} | FAILED |", sp, i + 1, name);
                failures += 1;
            }
        }
        
        println!("------------------------------------------------------------");
        println!("TEST SUMMARY");
        println!("Passed {}/{}", successes, successes + failures);
        println!("------------------------------------------------------------");
    }

    #[test]
    fn test_pawn_moves() {
        println!("\n=== WHITE PAWN TESTS ===");
        // Test cases for white pawns
        let white_pawn_tests = [
            // Test 1: White pawn on e2 can push 2 squares (no captures)
            ("WP push 2, no captures", 12, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![20, 28]),
            
            // Test 2: White pawn on e2 can only push 1 square (blocked by black pawn)
            ("WP push 1, no captures", 12, "rnbqkbnr/pppppppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![20, 0]),
            
            // Test 3: White pawn on e3 can only push 1 square (blocked by black pawn)
            ("WP push 1, no captures", 20, "rnbqkbnr/pppppppp/3p4/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1", vec![28, 0]),
            
            // Test 4: White pawn on e2 cannot move (blocked by black pawn)
            ("WP no push, no captures", 12, "rnbqkbnr/pppppppp/3p4/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![0]),
            
            // Test 5: White pawn on e2 can capture two black pawns
            ("WP no push, 2 captures", 12, "rnbqkbnr/pppppppp/2p1p3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![19, 21]),
            
            // Test 6: White pawn on h3 can capture one black pawn
            ("WP no push, 1 captures", 23, "rnbqkbnr/pppppppp/7p/8/8/7P/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![30]),
        ];

        // Run white pawn tests
        let test_names: Vec<&str> = white_pawn_tests.iter().map(|t| t.0).collect();
        let squares: Vec<u64> = white_pawn_tests.iter().map(|t| t.1).collect();
        let board_states: Vec<&str> = white_pawn_tests.iter().map(|t| t.2).collect();
        let expected_moves: Vec<Vec<u64>> = white_pawn_tests.iter().map(|t| t.3.clone()).collect();
        let expected_moves_refs: Vec<&[u64]> = expected_moves.iter().map(|v| v.as_slice()).collect();
        run_all_tests(&test_names, &squares, &board_states, &expected_moves_refs, true, true);

        println!("\n=== BLACK PAWN TESTS ===");
        // Test cases for black pawns
        let black_pawn_tests = [
            // Test 7: Black pawn on e7 can push 2 squares (no captures)
            ("BP push 2, no captures", 52, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![44, 36]),
            
            // Test 8: Black pawn on e7 can only push 1 square (blocked by white pawn)
            ("BP push 1, no captures", 52, "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1", vec![44, 0]),
            
            // Test 9: Black pawn on e6 can only push 1 square (blocked by white pawn)
            ("BP push 1, no captures", 44, "rnbqkbnr/ppppp1pp/4p3/8/3P4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![36, 0]),
            
            // Test 10: Black pawn on e7 cannot move (blocked by white pawn)
            ("BP no push, no captures", 52, "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1", vec![0]),
            
            // Test 11: Black pawn on e7 can capture two white pawns
            ("BP no push, 2 captures", 52, "rnbqkbnr/pppppppp/8/8/2P1P3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![43, 45]),
            
            // Test 12: Black pawn on h7 can capture one white pawn
            ("BP no push, 1 captures", 55, "rnbqkbnr/ppppppp1/7P/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![46]),
        ];

        // Run black pawn tests
        let test_names: Vec<&str> = black_pawn_tests.iter().map(|t| t.0).collect();
        let squares: Vec<u64> = black_pawn_tests.iter().map(|t| t.1).collect();
        let board_states: Vec<&str> = black_pawn_tests.iter().map(|t| t.2).collect();
        let expected_moves: Vec<Vec<u64>> = black_pawn_tests.iter().map(|t| t.3.clone()).collect();
        let expected_moves_refs: Vec<&[u64]> = expected_moves.iter().map(|v| v.as_slice()).collect();
        run_all_tests(&test_names, &squares, &board_states, &expected_moves_refs, false, true);
    }

    #[test]
    fn test_rook_moves() {
        let mut board = Bitboard::new();
        
        // Test white rook moves
        board.w_rook = 1 << 0; // a1
        let moves = get_rook_moves(&board, 0, true);
        assert_eq!(moves.len(), 14);
        assert!(moves.contains(&1)); // b1
        assert!(moves.contains(&8)); // a2
        
        // Test black rook moves
        board.clear();
        board.b_rook = 1 << 63; // h8
        let moves = get_rook_moves(&board, 63, false);
        assert_eq!(moves.len(), 14);
        assert!(moves.contains(&62)); // g8
        assert!(moves.contains(&55)); // h7
    }

    #[test]
    fn test_knight_moves() {
        let mut board = Bitboard::new();
        
        // Test white knight moves
        board.w_knight = 1 << 1; // b1
        let moves = get_knight_moves(&board, 1, true);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&16)); // a3
        assert!(moves.contains(&18)); // c3
        assert!(moves.contains(&11)); // d2
        
        // Test black knight moves
        board.clear();
        board.b_knight = 1 << 62; // g8
        let moves = get_knight_moves(&board, 62, false);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&47)); // f6
        assert!(moves.contains(&45)); // e6
        assert!(moves.contains(&52)); // e7
    }

    #[test]
    fn test_bishop_moves() {
        let mut board = Bitboard::new();
        
        // Test white bishop moves
        board.w_bishop = 1 << 2; // c1
        let moves = get_bishop_moves(&board, 2, true);
        assert_eq!(moves.len(), 7);
        assert!(moves.contains(&9)); // b2
        assert!(moves.contains(&11)); // d2
        assert!(moves.contains(&16)); // a3
        
        // Test black bishop moves
        board.clear();
        board.b_bishop = 1 << 61; // f8
        let moves = get_bishop_moves(&board, 61, false);
        assert_eq!(moves.len(), 7);
        assert!(moves.contains(&52)); // e7
        assert!(moves.contains(&54)); // g7
        assert!(moves.contains(&43)); // d6
    }

    #[test]
    fn test_queen_moves() {
        let mut board = Bitboard::new();
        
        // Test white queen moves
        board.w_queen = 1 << 3; // d1
        let moves = get_queen_moves(&board, 3, true);
        assert_eq!(moves.len(), 21);
        assert!(moves.contains(&2)); // c1
        assert!(moves.contains(&4)); // e1
        assert!(moves.contains(&11)); // d2
        assert!(moves.contains(&10)); // c2
        assert!(moves.contains(&12)); // e2
        
        // Test black queen moves
        board.clear();
        board.b_queen = 1 << 60; // e8
        let moves = get_queen_moves(&board, 60, false);
        assert_eq!(moves.len(), 21);
        assert!(moves.contains(&59)); // d8
        assert!(moves.contains(&61)); // f8
        assert!(moves.contains(&52)); // e7
        assert!(moves.contains(&51)); // d7
        assert!(moves.contains(&53)); // f7
    }

    #[test]
    fn test_king_moves() {
        let mut board = Bitboard::new();
        
        // Test white king moves
        board.w_king = 1 << 4; // e1
        let moves = get_king_moves(&board, 4, true);
        assert_eq!(moves.len(), 5);
        assert!(moves.contains(&3)); // d1
        assert!(moves.contains(&5)); // f1
        assert!(moves.contains(&11)); // d2
        assert!(moves.contains(&12)); // e2
        assert!(moves.contains(&13)); // f2
        
        // Test black king moves
        board.clear();
        board.b_king = 1 << 59; // d8
        let moves = get_king_moves(&board, 59, false);
        assert_eq!(moves.len(), 5);
        assert!(moves.contains(&58)); // c8
        assert!(moves.contains(&60)); // e8
        assert!(moves.contains(&50)); // c7
        assert!(moves.contains(&51)); // d7
        assert!(moves.contains(&52)); // e7
    }
}

fn square_to_algebraic(square: u64) -> String {
    let file = (square % 8) as u8 + b'a';
    let rank = (square / 8) as u8 + b'1';
    format!("{}{}", file as char, rank as char)
} 