use crate::bitboard::Bitboard;

pub fn get_pawn_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let occupied = board.get_occupied_squares();
    let enemy = if is_white { board.get_black_squares() } else { board.get_white_squares() };
    
    if is_white {
        // Check if pawn can push forward
        if square < 56 {
            let forward = 1u64 << (square + 8);
            let forward_two = 1u64 << (square + 16);
            let capture_left = if square % 8 > 0 { 1u64 << (square + 7) } else { 0 };
            let capture_right = if square % 8 < 7 { 1u64 << (square + 9) } else { 0 };
            
            // Check if blocked
            if (occupied & forward) != 0 {
                // Only check captures if blocked
                if capture_left != 0 && (enemy & capture_left) != 0 {
                    moves.push(square + 7);
                }
                if capture_right != 0 && (enemy & capture_right) != 0 {
                    moves.push(square + 9);
                }
                // If no captures and blocked, return [0]
                if moves.is_empty() {
                    return vec![0];
                }
            } else {
                // Check if blocked by a piece two squares ahead
                let blocked_two = square >= 8 && square < 16 && (occupied & forward_two) != 0;
                
                // Check captures first
                if capture_left != 0 && (enemy & capture_left) != 0 {
                    moves.push(square + 7);
                }
                if capture_right != 0 && (enemy & capture_right) != 0 {
                    moves.push(square + 9);
                }
                
                // Then add pushes
                moves.push(square + 8);
                
                // Add double push if not blocked
                if !blocked_two && square >= 8 && square < 16 {
                    moves.push(square + 16);
                }
            }
        }
    } else {
        // Check if pawn can push forward
        if square > 7 {
            let forward = 1u64 << (square - 8);
            let forward_two = 1u64 << (square - 16);
            let capture_left = if square % 8 > 0 { 1u64 << (square - 9) } else { 0 };
            let capture_right = if square % 8 < 7 { 1u64 << (square - 7) } else { 0 };
            
            // Check if blocked
            if (occupied & forward) != 0 {
                // Only check captures if blocked
                if capture_left != 0 && (enemy & capture_left) != 0 {
                    moves.push(square - 9);
                }
                if capture_right != 0 && (enemy & capture_right) != 0 {
                    moves.push(square - 7);
                }
                // If no captures and blocked, return [0]
                if moves.is_empty() {
                    return vec![0];
                }
            } else {
                // Check if blocked by a piece two squares ahead
                let blocked_two = square >= 48 && square < 56 && (occupied & forward_two) != 0;
                
                // Check captures first
                if capture_left != 0 && (enemy & capture_left) != 0 {
                    moves.push(square - 9);
                }
                if capture_right != 0 && (enemy & capture_right) != 0 {
                    moves.push(square - 7);
                }
                
                // Then add pushes
                moves.push(square - 8);
                
                // Add double push if not blocked
                if !blocked_two && square >= 48 && square < 56 {
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

pub fn get_knight_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let friendly = if is_white { board.get_white_squares() } else { board.get_black_squares() };
    
    let offsets = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1)
    ];
    
    let x = (square % 8) as i32;
    let y = (square / 8) as i32;
    
    for (dx, dy) in offsets.iter() {
        let new_x = x + dx;
        let new_y = y + dy;
        
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let target = (new_y * 8 + new_x) as u64;
            if (friendly & (1 << target)) == 0 {
                moves.push(target);
            }
        }
    }
    
    moves
}

pub fn get_bishop_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let occupied = board.get_occupied_squares();
    let friendly = if is_white { board.get_white_squares() } else { board.get_black_squares() };
    
    // Directions: up-right, down-right, down-left, up-left
    let directions = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    
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

pub fn get_queen_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    moves.extend(get_rook_moves(board, square, is_white));
    moves.extend(get_bishop_moves(board, square, is_white));
    moves
}

pub fn get_king_moves(board: &Bitboard, square: u64, is_white: bool) -> Vec<u64> {
    let mut moves = Vec::new();
    let friendly = if is_white { board.get_white_squares() } else { board.get_black_squares() };
    
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];
    
    let x = (square % 8) as i32;
    let y = (square / 8) as i32;
    
    for (dx, dy) in offsets.iter() {
        let new_x = x + dx;
        let new_y = y + dy;
        
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let target = (new_y * 8 + new_x) as u64;
            if (friendly & (1 << target)) == 0 {
                moves.push(target);
            }
        }
    }
    
    moves
} 