package chess

import (
	"strings"
	"testing"
)

func TestInitialPosition(t *testing.T) {
	bb := InitBitboard()

	// Test initial piece positions
	if bb.Wpawn != uint64(255)<<48 {
		t.Error("White pawns not in correct starting position")
	}
	if bb.Bpawn != uint64(255)<<8 {
		t.Error("Black pawns not in correct starting position")
	}

	// Test piece counting
	occupied := bb.GetOccupiedSquares()
	count := 0
	for i := uint64(0); i < 64; i++ {
		if occupied>>i&1 == 1 {
			count++
		}
	}
	if count != 32 {
		t.Errorf("Expected 32 pieces on board, got %d", count)
	}
}

func TestPawnMoves(t *testing.T) {
	bb := InitBitboard()

	// Test white pawn moves from e2
	moves := bb.GetPawnMoves(12) // e2 square
	if len(moves) != 2 {
		t.Errorf("Expected 2 moves for white pawn on e2, got %d", len(moves))
	}

	// Test black pawn moves from e7
	moves = bb.GetPawnMoves(52) // e7 square
	if len(moves) != 2 {
		t.Errorf("Expected 2 moves for black pawn on e7, got %d", len(moves))
	}
}

func TestRookMoves(t *testing.T) {
	bb := InitBitboard()

	// Test white rook moves from a1
	moves := bb.GetRookMoves(0) // a1 square
	if len(moves) != 0 {
		t.Errorf("Expected 0 moves for white rook on a1 (blocked), got %d", len(moves))
	}

	// Clear the board and place a single white rook on e4
	bb.ClearBitboard()
	bb.Wrook = uint64(1) << (63 - 28) // e4 square

	moves = bb.GetRookMoves(28) // e4 square
	expectedMoves := 14         // 7 horizontal + 7 vertical moves
	if len(moves) != expectedMoves {
		t.Errorf("Expected %d moves for white rook on e4, got %d", expectedMoves, len(moves))
	}
}

func TestFENConversion(t *testing.T) {
	testCases := []struct {
		name string
		fen  string
	}{
		{
			name: "Initial position",
			fen:  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
		},
		{
			name: "Empty board",
			fen:  "8/8/8/8/8/8/8/8 w - - 0 1",
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			bb := &Bitboard{}
			bb.FromFEN(tc.fen)

			// Convert back to FEN and compare the piece placement part
			resultFEN := bb.ToFEN()
			resultPieces := resultFEN[:strings.Index(resultFEN, " ")]
			expectedPieces := tc.fen[:strings.Index(tc.fen, " ")]

			if resultPieces != expectedPieces {
				t.Errorf("FEN conversion failed\nExpected: %s\nGot: %s", expectedPieces, resultPieces)
			}
		})
	}
}

func TestAlgebraicNotation(t *testing.T) {
	testCases := []struct {
		numeric   int
		algebraic string
	}{
		{0, "a1"},
		{7, "h1"},
		{56, "a8"},
		{63, "h8"},
		{28, "e4"},
	}

	for _, tc := range testCases {
		result := NumericToAlgebraic(tc.numeric)
		if result != tc.algebraic {
			t.Errorf("NumericToAlgebraic(%d) = %s, want %s", tc.numeric, result, tc.algebraic)
		}

		numeric := AlgebraicToNumeric(tc.algebraic)
		if numeric != tc.numeric {
			t.Errorf("AlgebraicToNumeric(%s) = %d, want %d", tc.algebraic, numeric, tc.numeric)
		}
	}
}
