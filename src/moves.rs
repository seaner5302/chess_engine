use crate::bitboard::Bitboard;

pub fn get_pawn_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let occupied = board.get_occupied_squares();
    let enemy = if is_white { board.get_black_squares() } else { board.get_white_squares() };
    
    if is_white {
        if square < 56 {  // Not on the last rank
            let file = square % 8;
            let rank = square / 8;
            let one_forward = square + 8;
            
            // Check forward moves
            if one_forward < 64 {
                let mut path_blocked = false;
                
                // Check if the immediate forward square is blocked
                if (occupied & (1u64 << one_forward)) != 0 {
                    path_blocked = true;
                }
                
                // Check if any enemy pieces control the path
                if !path_blocked {
                    for f in (file.saturating_sub(1))..=(file + 1).min(7) {
                        let check_square = (rank + 1) * 8 + f;
                        if check_square < 64 && (enemy & (1u64 << check_square)) != 0 {
                            path_blocked = true;
                            break;
                        }
                    }
                }
                
                if !path_blocked {
                    moves.push(one_forward);
                    
                    // Check double push from starting position (rank 2)
                    if square >= 8 && square <= 15 {
                        let two_forward = square + 16;
                        let mut path_blocked = false;
                        
                        // Check if the two-forward square is blocked
                        if two_forward < 64 && (occupied & (1u64 << two_forward)) != 0 {
                            path_blocked = true;
                        }
                        
                        // Check if any enemy pieces control the path
                        if !path_blocked {
                            for r in rank+1..=rank+2 {
                                for f in (file.saturating_sub(1))..=(file + 1).min(7) {
                                    let check_square = r * 8 + f;
                                    if check_square < 64 && (enemy & (1u64 << check_square)) != 0 {
                                        path_blocked = true;
                                        break;
                                    }
                                }
                                if path_blocked {
                                    break;
                                }
                            }
                        }
                        
                        if !path_blocked {
                            moves.push(two_forward);
                        }
                    }
                }
            }
            
            // Check captures
            if file > 0 {  // Not on a-file
                let left_capture = square + 7;
                if left_capture < 64 {
                    let target_bit = 1u64 << left_capture;
                    // Only allow capture if there's an enemy piece to capture
                    if (enemy & target_bit) != 0 {
                        moves.push(left_capture);
                    }
                }
            }
            if file < 7 {  // Not on h-file
                let right_capture = square + 9;
                if right_capture < 64 {
                    let target_bit = 1u64 << right_capture;
                    // Only allow capture if there's an enemy piece to capture
                    if (enemy & target_bit) != 0 {
                        moves.push(right_capture);
                    }
                }
            }
        }
    } else {
        if square > 7 {  // Not on the first rank
            let file = square % 8;
            let rank = square / 8;
            let one_forward = square - 8;
            
            // Check forward moves
            let mut path_blocked = false;
            
            // Check if the immediate forward square is blocked
            if (occupied & (1u64 << one_forward)) != 0 {
                path_blocked = true;
            }
            
            // Check if any enemy pieces control the path
            if !path_blocked {
                for f in (file.saturating_sub(1))..=(file + 1).min(7) {
                    let check_square = (rank - 1) * 8 + f;
                    if (enemy & (1u64 << check_square)) != 0 {
                        path_blocked = true;
                        break;
                    }
                }
            }
            
            if !path_blocked {
                moves.push(one_forward);
                
                // Check double push from starting position (rank 7)
                if square >= 48 && square < 56 {
                    let two_forward = square - 16;
                    let mut path_blocked = false;
                    
                    // Check if the two-forward square is blocked
                    if (occupied & (1u64 << two_forward)) != 0 {
                        path_blocked = true;
                    }
                    
                    // Check if any enemy pieces control the path
                    if !path_blocked {
                        for r in ((rank-2)..=rank-1).rev() {
                            for f in (file.saturating_sub(1))..=(file + 1).min(7) {
                                let check_square = r * 8 + f;
                                if (enemy & (1u64 << check_square)) != 0 {
                                    path_blocked = true;
                                    break;
                                }
                            }
                            if path_blocked {
                                break;
                            }
                        }
                    }
                    
                    if !path_blocked {
                        moves.push(two_forward);
                    }
                }
            }
            
            // Check captures
            if file > 0 {  // Not on a-file
                let left_capture = square - 9;
                let target_bit = 1u64 << left_capture;
                // Only allow capture if there's an enemy piece to capture
                if (enemy & target_bit) != 0 {
                    moves.push(left_capture);
                }
            }
            if file < 7 {  // Not on h-file
                let right_capture = square - 7;
                let target_bit = 1u64 << right_capture;
                // Only allow capture if there's an enemy piece to capture
                if (enemy & target_bit) != 0 {
                    moves.push(right_capture);
                }
            }
        }
    }
    
    // Sort moves numerically
    moves.sort_unstable();
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