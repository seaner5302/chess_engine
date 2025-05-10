mod bitboard;
mod moves;
mod tests;

use bitboard::Bitboard;
use moves::*;

fn main() {
    let mut board = Bitboard::new();
    board.init();
    
    println!("Initial board state:");
    println!("{}", board.to_string());
    
    // Example: Get moves for white pawn at e2 (square 12)
    let pawn_moves = get_pawn_moves(&board, 12, true);
    println!("\nPossible moves for white pawn at e2:");
    for square in pawn_moves {
        println!("{}", square_to_algebraic(square));
    }
    
    // Example: Get moves for white rook at a1 (square 0)
    let rook_moves = get_rook_moves(&board, 0, true);
    println!("\nPossible moves for white rook at a1:");
    for square in rook_moves {
        println!("{}", square_to_algebraic(square));
    }
}

fn square_to_algebraic(square: u64) -> String {
    let file = (square % 8) as u8 + b'a';
    let rank = (square / 8) as u8 + b'1';
    format!("{}{}", file as char, rank as char)
} 