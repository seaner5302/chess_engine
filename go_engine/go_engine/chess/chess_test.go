package chess

import (
	"reflect"
	"sort"
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
	// cSquareToGoSquare converts C-style 1-64 square (a1=1, h8=64) to Go-style 0-63 square (a1=0, h8=63)
	cSquareToGoSquare := func(cSquare int) int {
		if cSquare < 1 || cSquare > 64 {
			return -1 // Indicate an invalid C square
		}
		return cSquare - 1
	}

	type pawnTestCase struct {
		name              string
		fen               string
		cStartSquare      int   // C-style 1-64
		cExpectedCSquares []int // C-style 1-64, 0 terminates
	}

	testCases := []pawnTestCase{
		{"WP push 2, no captures", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 12, []int{20, 28}},
		{"WP push 1, no captures", "rnbqkbnr/3p4/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 12, []int{20, 0}},
		{"WP push 1, no captures", "rnbqkbnr/8/3p4/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 20, []int{28, 0}},
		{"WP no push, no captures", "rnbqkbnr/3p4/3p4/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq 0 0", 12, []int{0, 0}},
		{"WP no push, 2 captures", "rnbqkbnr/3p4/2ppp3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 12, []int{19, 21}},
		{"WP no push, 1 captures", "rnbqkbnr/pppppppp/7p/8/8/7P/PPPPPPPP/RNBQKBNR w KQkq 0 0", 23, []int{30, 0}},
		{"BP push 2, no captures", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 52, []int{44, 36}},
		{"BP push 1, no captures", "rnbqkbnr/pppppppp/8/8/3p4/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", 52, []int{44, 0}},
		{"BP push 1, no captures", "rnbqkbnr/pppppppp/8/8/8/3P4/PPPPPPPP/RNBQKBNR w KQkq 0 0", 44, []int{36, 0}},
		{"BP no push, no captures", "rnbqkbnr/pppppppp/8/8/8/3P4/PPPPPPPP/RNBQKBNR w KQkq 0 0", 52, []int{0, 0}},
		{"BP no push, 2 captures", "rnbqkbnr/pppppppp/8/8/8/2ppp3/PPPPPPPP/RNBQKBNR w KQkq 0 0", 52, []int{45, 43}},
		{"BP no push, 1 captures", "rnbqkbnr/pppppppp/8/8/8/6pp/7P/RNBQKBNR w KQkq 0 0", 56, []int{47, 0}},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			bb := &Bitboard{}
			err := bb.FromFEN(tc.fen)
			if err != nil {
				t.Fatalf("Failed to parse FEN '%s': %v", tc.fen, err)
			}

			goStartSquare := cSquareToGoSquare(tc.cStartSquare)
			if goStartSquare == -1 {
				t.Fatalf("Invalid C start square %d for test '%s'", tc.cStartSquare, tc.name)
			}

			t.Logf("Board state for test '%s' (Pawn on C sq: %d, Go sq: %s):\n%s", tc.name, tc.cStartSquare, NumericToAlgebraic(goStartSquare), bb.String())

			actualMoveStructs := bb.GetPawnMoves(goStartSquare)
			actualToSquares := make([]int, 0, len(actualMoveStructs))
			for _, move := range actualMoveStructs {
				actualToSquares = append(actualToSquares, move.To)
			}
			sort.Ints(actualToSquares)

			expectedToSquares := make([]int, 0, len(tc.cExpectedCSquares))
			for _, cMoveCSquare := range tc.cExpectedCSquares {
				if cMoveCSquare == 0 {
					break // C uses 0 to terminate the list of expected moves
				}
				goMoveSquare := cSquareToGoSquare(cMoveCSquare)
				if goMoveSquare == -1 {
					t.Errorf("Invalid C expected move square %d for test '%s'", cMoveCSquare, tc.name)
					continue
				}
				expectedToSquares = append(expectedToSquares, goMoveSquare)
			}
			sort.Ints(expectedToSquares)

			if !reflect.DeepEqual(actualToSquares, expectedToSquares) {
				startAlg := NumericToAlgebraic(goStartSquare)
				actualAlg := make([]string, len(actualToSquares))
				for i, s := range actualToSquares {
					actualAlg[i] = NumericToAlgebraic(s)
				}
				expectedAlg := make([]string, len(expectedToSquares))
				for i, s := range expectedToSquares {
					expectedAlg[i] = NumericToAlgebraic(s)
				}
				t.Errorf("TestPawnMoves failed for: %s\nFEN: %s\nPawn on: %s (Go sq: %d, C sq: %d)\nExpected target squares (Go): %v (alg: %v)\nActual target squares (Go):   %v (alg: %v)",
					tc.name, tc.fen, startAlg, goStartSquare, tc.cStartSquare,
					expectedToSquares, expectedAlg,
					actualToSquares, actualAlg)
			}
			t.Fail() // TEMPORARY: Force sub-test to fail to show t.Logf output with -v
		})
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
