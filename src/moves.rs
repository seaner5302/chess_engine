use crate::bitboard::Bitboard;

pub fn get_pawn_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let occupied = board.get_occupied_squares();
    let enemy = if is_white { board.get_black_squares() } else { board.get_white_squares() };
    
    if is_white {
        // Check if pawn can push forward
        if square < 56 {
            // Check if blocked
            if (occupied & (1u64 << (square + 8))) != 0 {
                // Check captures only
                if square % 8 > 0 && (enemy & (1u64 << (square + 7))) != 0 {
                    moves.push(square + 7);
                }
                if square % 8 < 7 && (enemy & (1u64 << (square + 9))) != 0 {
                    moves.push(square + 9);
                }
                // If no captures and blocked, return [0]
                if moves.is_empty() {
                    return vec![0];
                }
            } else {
                // Check captures first
                if square % 8 > 0 && (enemy & (1u64 << (square + 7))) != 0 {
                    moves.push(square + 7);
                }
                if square % 8 < 7 && (enemy & (1u64 << (square + 9))) != 0 {
                    moves.push(square + 9);
                }
                
                // Then add pushes
                moves.push(square + 8);
                
                // Double push from starting position if not blocked
                if square >= 8 && square < 16 && (occupied & (1u64 << (square + 16))) == 0 {
                    moves.push(square + 16);
                }
            }
        }
    } else {
        // Check if pawn can push forward
        if square > 7 {
            // Check if blocked
            if (occupied & (1u64 << (square - 8))) != 0 {
                // Check captures only
                if square % 8 > 0 && (enemy & (1u64 << (square - 9))) != 0 {
                    moves.push(square - 9);
                }
                if square % 8 < 7 && (enemy & (1u64 << (square - 7))) != 0 {
                    moves.push(square - 7);
                }
                // If no captures and blocked, return [0]
                if moves.is_empty() {
                    return vec![0];
                }
            } else {
                // Check captures first
                if square % 8 > 0 && (enemy & (1u64 << (square - 9))) != 0 {
                    moves.push(square - 9);
                }
                if square % 8 < 7 && (enemy & (1u64 << (square - 7))) != 0 {
                    moves.push(square - 7);
                }
                
                // Then add pushes
                moves.push(square - 8);
                
                // Double push from starting position if not blocked
                if square >= 48 && square < 56 && (occupied & (1u64 << (square - 16))) == 0 {
                    moves.push(square - 16);
                }
            }
        }
    }
    
    // Sort moves numerically
    moves.sort_unstable();
    
    // If no valid moves, return vec![0]
    if moves.is_empty() {
        return vec![0];
    }
    
    moves
}

pub fn get_rook_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let occupied = board.get_occupied_squares();
    let friendly = if is_white { board.get_white_squares() } else { board.get_black_squares() };
    
    // Directions: up, right, down, left
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for (dx, dy) in directions.iter() {
        let mut x = (square % 8) as i32;
        let mut y = (square / 8) as i32;
        
        loop {
            x += dx;
            y += dy;
            
            if x < 0 || x > 7 || y < 0 || y > 7 {
                break;
            }
            
            let target = (y * 8 + x) as u64;
            let target_bit = 1 << target;
            
            if (occupied & target_bit) == 0 {
                moves.push(target);
            } else {
                if (friendly & target_bit) == 0 {
                    moves.push(target);
                }
                break;
            }
        }
    }
    
    moves
}