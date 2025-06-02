package chess

// Bitboard represents a chess position using bitboards for each piece type
type Bitboard struct {
	Wpawn   uint64
	Wknight uint64
	Wbishop uint64
	Wrook   uint64
	Wqueen  uint64
	Wking   uint64

	Bpawn   uint64
	Bknight uint64
	Bbishop uint64
	Brook   uint64
	Bqueen  uint64
	Bking   uint64
}

// PieceNames maps piece types to their FEN notation characters
var PieceNames = []rune{'p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'}

// InitBitboard initializes a new bitboard with pieces in their starting positions
func InitBitboard() *Bitboard {
	bb := &Bitboard{}
	bb.Wpawn = uint64(255) << 48  // White pawns on rank 2
	bb.Wknight = uint64(66) << 56 // White knights on b1 and g1
	bb.Wbishop = uint64(36) << 56 // White bishops on c1 and f1
	bb.Wrook = uint64(129) << 56  // White rooks on a1 and h1
	bb.Wqueen = uint64(8) << 56   // White queen on d1
	bb.Wking = uint64(16) << 56   // White king on e1

	bb.Bpawn = uint64(255) << 8 // Black pawns on rank 7
	bb.Bknight = uint64(66)     // Black knights on b8 and g8
	bb.Bbishop = uint64(36)     // Black bishops on c8 and f8
	bb.Brook = uint64(129)      // Black rooks on a8 and h8
	bb.Bqueen = uint64(8)       // Black queen on d8
	bb.Bking = uint64(16)       // Black king on e8

	return bb
}

// ClearBitboard sets all bitboards to 0
func (bb *Bitboard) ClearBitboard() {
	bb.Wpawn = 0
	bb.Wknight = 0
	bb.Wbishop = 0
	bb.Wrook = 0
	bb.Wqueen = 0
	bb.Wking = 0

	bb.Bpawn = 0
	bb.Bknight = 0
	bb.Bbishop = 0
	bb.Brook = 0
	bb.Bqueen = 0
	bb.Bking = 0
}

// GetOccupiedSquares returns a bitboard with all occupied squares
func (bb *Bitboard) GetOccupiedSquares() uint64 {
	return bb.Wpawn | bb.Wknight | bb.Wbishop | bb.Wrook | bb.Wqueen | bb.Wking |
		bb.Bpawn | bb.Bknight | bb.Bbishop | bb.Brook | bb.Bqueen | bb.Bking
}

// GetEmptySquares returns a bitboard with all empty squares
func (bb *Bitboard) GetEmptySquares() uint64 {
	return ^bb.GetOccupiedSquares()
}

// GetWhiteSquares returns a bitboard with all squares occupied by white pieces
func (bb *Bitboard) GetWhiteSquares() uint64 {
	return bb.Wpawn | bb.Wknight | bb.Wbishop | bb.Wrook | bb.Wqueen | bb.Wking
}

// GetBlackSquares returns a bitboard with all squares occupied by black pieces
func (bb *Bitboard) GetBlackSquares() uint64 {
	return bb.Bpawn | bb.Bknight | bb.Bbishop | bb.Brook | bb.Bqueen | bb.Bking
}

// String returns a string representation of the board
func (bb *Bitboard) String() string {
	board := make([]rune, 71)
	for i := range board {
		if (i+1)%9 == 0 {
			board[i] = '\n'
		} else {
			board[i] = '-'
		}
	}

	for square := 0; square < 64; square++ {
		var piece rune
		switch {
		case (bb.Wpawn>>square)&1 == 1:
			piece = 'p'
		case (bb.Wknight>>square)&1 == 1:
			piece = 'n'
		case (bb.Wbishop>>square)&1 == 1:
			piece = 'b'
		case (bb.Wrook>>square)&1 == 1:
			piece = 'r'
		case (bb.Wqueen>>square)&1 == 1:
			piece = 'q'
		case (bb.Wking>>square)&1 == 1:
			piece = 'k'
		case (bb.Bpawn>>square)&1 == 1:
			piece = 'P'
		case (bb.Bknight>>square)&1 == 1:
			piece = 'N'
		case (bb.Bbishop>>square)&1 == 1:
			piece = 'B'
		case (bb.Brook>>square)&1 == 1:
			piece = 'R'
		case (bb.Bqueen>>square)&1 == 1:
			piece = 'Q'
		case (bb.Bking>>square)&1 == 1:
			piece = 'K'
		default:
			continue
		}
		board[square+square/8] = piece
	}

	return string(board)
}
