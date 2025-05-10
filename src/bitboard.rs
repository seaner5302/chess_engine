#[derive(Debug, Clone, Copy)]
pub struct Bitboard {
    pub w_pawn: u64,
    pub w_knight: u64,
    pub w_bishop: u64,
    pub w_rook: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub b_pawn: u64,
    pub b_knight: u64,
    pub b_bishop: u64,
    pub b_rook: u64,
    pub b_queen: u64,
    pub b_king: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        Bitboard {
            w_pawn: 0,
            w_knight: 0,
            w_bishop: 0,
            w_rook: 0,
            w_queen: 0,
            w_king: 0,
            b_pawn: 0,
            b_knight: 0,
            b_bishop: 0,
            b_rook: 0,
            b_queen: 0,
            b_king: 0,
        }
    }

    pub fn init(&mut self) {
        self.w_pawn = 0xFF << 48;
        self.w_knight = 0x42 << 56;
        self.w_bishop = 0x24 << 56;
        self.w_rook = 0x81 << 56;
        self.w_queen = 0x08 << 56;
        self.w_king = 0x10 << 56;

        self.b_pawn = 0xFF << 8;
        self.b_knight = 0x42;
        self.b_bishop = 0x24;
        self.b_rook = 0x81;
        self.b_queen = 0x08;
        self.b_king = 0x10;
    }

    pub fn clear(&mut self) {
        *self = Bitboard::new();
    }

    pub fn get_occupied_squares(&self) -> u64 {
        self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | self.w_queen | self.w_king |
        self.b_pawn | self.b_knight | self.b_bishop | self.b_rook | self.b_queen | self.b_king
    }

    pub fn get_empty_squares(&self) -> u64 {
        !self.get_occupied_squares()
    }

    pub fn get_white_squares(&self) -> u64 {
        self.w_pawn | self.w_knight | self.w_bishop | self.w_rook | self.w_queen | self.w_king
    }

    pub fn get_black_squares(&self) -> u64 {
        self.b_pawn | self.b_knight | self.b_bishop | self.b_rook | self.b_queen | self.b_king
    }

    pub fn to_string(&self) -> String {
        let mut board = vec!['-'; 72];
        for i in 0..8 {
            board[i * 9 + 8] = '\n';
        }

        let piece_names = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];
        let pieces = [
            self.w_pawn, self.w_knight, self.w_bishop, self.w_rook, self.w_queen, self.w_king,
            self.b_pawn, self.b_knight, self.b_bishop, self.b_rook, self.b_queen, self.b_king,
        ];

        for (piece, &bitboard) in piece_names.iter().zip(pieces.iter()) {
            for i in 0..64 {
                if (bitboard >> i) & 1 == 1 {
                    let rank = 7 - (i / 8);
                    let file = i % 8;
                    let pos = rank * 9 + file;
                    board[pos] = *piece;
                }
            }
        }

        board.iter().collect()
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Bitboard::new();
        let mut parts = fen.split_whitespace();
        let position = parts.next().unwrap();
        let ranks: Vec<&str> = position.split('/').collect();
        
        for (rank_idx, rank) in ranks.iter().enumerate() {
            let mut file = 0;
            for c in rank.chars() {
                if c.is_digit(10) {
                    file += c.to_digit(10).unwrap() as usize;
                } else {
                    let square = (rank_idx * 8 + file) as u64;
                    match c {
                        'P' => board.w_pawn |= 1 << square,
                        'N' => board.w_knight |= 1 << square,
                        'B' => board.w_bishop |= 1 << square,
                        'R' => board.w_rook |= 1 << square,
                        'Q' => board.w_queen |= 1 << square,
                        'K' => board.w_king |= 1 << square,
                        'p' => board.b_pawn |= 1 << square,
                        'n' => board.b_knight |= 1 << square,
                        'b' => board.b_bishop |= 1 << square,
                        'r' => board.b_rook |= 1 << square,
                        'q' => board.b_queen |= 1 << square,
                        'k' => board.b_king |= 1 << square,
                        _ => {}
                    }
                    file += 1;
                }
            }
        }
        board
    }
} 