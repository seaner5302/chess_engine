#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::Bitboard;
    use crate::moves::*;

    #[test]
    fn test_pawn_moves() {
        eprintln!("\n=== WHITE PAWN TESTS ===");
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
        
        for i in 0..test_names.len() {
            let board = Bitboard::from_fen(board_states[i]);
            let moves = get_pawn_moves(&board, squares[i], true);
            let board_str = board.to_string();
            
            eprintln!("\nTest Name: {}", test_names[i]);
            eprintln!("Board State:");
            eprintln!("{}", board_str);
            eprintln!("Getting moves for white pawn on {}", square_to_algebraic(squares[i]));
            eprint!("Found moves: [");
            
            for (j, &mv) in moves.iter().enumerate() {
                eprint!("{} {}", mv, square_to_algebraic(mv));
                if j != moves.len() - 1 {
                    eprint!(", ");
                }
            }
            eprintln!("]");
            
            let valid = if expected_moves[i].len() == 1 && expected_moves[i][0] == 0 {
                moves.is_empty()
            } else {
                moves == expected_moves[i]
            };
            
            if valid {
                eprintln!("✓ Test PASSED");
            } else {
                eprintln!("✗ Test FAILED");
                eprint!("Expected moves: [");
                for (j, &mv) in expected_moves[i].iter().enumerate() {
                    if mv != 0 {
                        eprint!("{} {}", mv, square_to_algebraic(mv));
                        if j != expected_moves[i].len() - 1 {
                            eprint!(", ");
                        }
                    }
                }
                eprintln!("]");
            }
            eprintln!("------------------------------------------------------------");
        }

        eprintln!("\n=== BLACK PAWN TESTS ===");
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
        
        for i in 0..test_names.len() {
            let board = Bitboard::from_fen(board_states[i]);
            let moves = get_pawn_moves(&board, squares[i], false);
            let board_str = board.to_string();
            
            eprintln!("\nTest Name: {}", test_names[i]);
            eprintln!("Board State:");
            eprintln!("{}", board_str);
            eprintln!("Getting moves for black pawn on {}", square_to_algebraic(squares[i]));
            eprint!("Found moves: [");
            
            for (j, &mv) in moves.iter().enumerate() {
                eprint!("{} {}", mv, square_to_algebraic(mv));
                if j != moves.len() - 1 {
                    eprint!(", ");
                }
            }
            eprintln!("]");
            
            let valid = if expected_moves[i].len() == 1 && expected_moves[i][0] == 0 {
                moves.is_empty()
            } else {
                moves == expected_moves[i]
            };
            
            if valid {
                eprintln!("✓ Test PASSED");
            } else {
                eprintln!("✗ Test FAILED");
                eprint!("Expected moves: [");
                for (j, &mv) in expected_moves[i].iter().enumerate() {
                    if mv != 0 {
                        eprint!("{} {}", mv, square_to_algebraic(mv));
                        if j != expected_moves[i].len() - 1 {
                            eprint!(", ");
                        }
                    }
                }
                eprintln!("]");
            }
            eprintln!("------------------------------------------------------------");
        }
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
}

#[allow(dead_code)]
fn square_to_algebraic(square: u64) -> String {
    let file = (square % 8) as u8 + b'a';
    let rank = (square / 8) as u8 + b'1';
    format!("{}{}", file as char, rank as char)
} 