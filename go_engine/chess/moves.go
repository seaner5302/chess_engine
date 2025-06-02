package chess

import (
	"fmt"
)

// Move represents a chess move from one square to another
type Move struct {
	From int
	To   int
}

// NumericToAlgebraic converts a numeric square (0-63) to algebraic notation (e.g. "e4")
func NumericToAlgebraic(numeric int) string {
	if numeric < 0 || numeric > 63 {
		return "  "
	}
	rank := numeric / 8
	file := numeric % 8
	return string(rune('a'+file)) + string(rune('1'+rank))
}

// AlgebraicToNumeric converts algebraic notation (e.g. "e4") to a numeric square (0-63)
func AlgebraicToNumeric(alge string) int {
	if len(alge) != 2 {
		return -1
	}
	file := int(alge[0] - 'a')
	rank := int(alge[1] - '1')
	if file < 0 || file > 7 || rank < 0 || rank > 7 {
		return -1
	}
	return rank*8 + file
}

// GetPieceType returns the type of piece on a given square (1=pawn, 2=knight, etc.)
func (bb *Bitboard) GetPieceType(square int) int {
	flippedSquare := 63 - square
	switch {
	case (bb.Wpawn>>flippedSquare&1 == 1) || (bb.Bpawn>>flippedSquare&1 == 1):
		return 1
	case (bb.Wknight>>flippedSquare&1 == 1) || (bb.Bknight>>flippedSquare&1 == 1):
		return 2
	case (bb.Wbishop>>flippedSquare&1 == 1) || (bb.Bbishop>>flippedSquare&1 == 1):
		return 3
	case (bb.Wrook>>flippedSquare&1 == 1) || (bb.Brook>>flippedSquare&1 == 1):
		return 4
	case (bb.Wqueen>>flippedSquare&1 == 1) || (bb.Bqueen>>flippedSquare&1 == 1):
		return 5
	case (bb.Wking>>flippedSquare&1 == 1) || (bb.Bking>>flippedSquare&1 == 1):
		return 6
	default:
		return 0
	}
}

// GetPieceColor returns the color of the piece on a given square (1=white, 0=black, -1=empty)
func (bb *Bitboard) GetPieceColor(square int) int {
	flippedSquare := 63 - square
	if (bb.Wpawn>>flippedSquare&1 == 1) || (bb.Wknight>>flippedSquare&1 == 1) ||
		(bb.Wbishop>>flippedSquare&1 == 1) || (bb.Wrook>>flippedSquare&1 == 1) ||
		(bb.Wqueen>>flippedSquare&1 == 1) || (bb.Wking>>flippedSquare&1 == 1) {
		return 1
	}
	if (bb.Bpawn>>flippedSquare&1 == 1) || (bb.Bknight>>flippedSquare&1 == 1) ||
		(bb.Bbishop>>flippedSquare&1 == 1) || (bb.Brook>>flippedSquare&1 == 1) ||
		(bb.Bqueen>>flippedSquare&1 == 1) || (bb.Bking>>flippedSquare&1 == 1) {
		return 0
	}
	return -1
}

// GetPawnMoves returns all legal pawn moves from a given square
func (bb *Bitboard) GetPawnMoves(square int) []Move {
	moves := make([]Move, 0, 4)
	flippedSquare := 63 - square

	if bb.GetPieceType(square) != 1 {
		fmt.Printf("Square %d does not contain a pawn\n", square)
		return moves
	}

	// White pawn moves
	if bb.GetPieceColor(square) == 1 {
		// Single push forward
		targetSquare := flippedSquare - 8
		if targetSquare >= 0 && bb.GetOccupiedSquares()>>targetSquare&1 == 0 {
			moves = append(moves, Move{From: square, To: square + 8})

			// Double push from starting rank
			if square >= 8 && square <= 15 {
				targetSquare = flippedSquare - 16
				if targetSquare >= 0 && bb.GetOccupiedSquares()>>targetSquare&1 == 0 {
					moves = append(moves, Move{From: square, To: square + 16})
				}
			}
		}

		// Captures
		if (square+1)%8 != 0 { // Not on h-file
			targetSquare := flippedSquare - 7
			if targetSquare >= 0 && targetSquare < 64 &&
				bb.GetPieceColor(63-targetSquare) == 0 &&
				bb.GetPieceType(63-targetSquare) != 0 {
				moves = append(moves, Move{From: square, To: square + 9})
			}
		}
		if (square+1)%8 != 1 { // Not on a-file
			targetSquare := flippedSquare - 9
			if targetSquare >= 0 && targetSquare < 64 &&
				bb.GetPieceColor(63-targetSquare) == 0 &&
				bb.GetPieceType(63-targetSquare) != 0 {
				moves = append(moves, Move{From: square, To: square + 7})
			}
		}
	} else { // Black pawn moves
		// Single push forward
		if bb.GetOccupiedSquares()>>(flippedSquare+8)&1 == 0 {
			moves = append(moves, Move{From: square, To: square - 8})

			// Double push from starting rank
			if square >= 48 && square <= 55 &&
				bb.GetOccupiedSquares()>>(flippedSquare+16)&1 == 0 {
				moves = append(moves, Move{From: square, To: square - 16})
			}
		}

		// Captures
		if (square+1)%8 != 0 { // Not on h-file
			targetSquare := flippedSquare + 9
			if targetSquare >= 0 && targetSquare < 64 &&
				bb.GetPieceColor(63-targetSquare) == 1 &&
				bb.GetPieceType(63-targetSquare) != 0 {
				moves = append(moves, Move{From: square, To: square - 7})
			}
		}
		if (square+1)%8 != 1 { // Not on a-file
			targetSquare := flippedSquare + 7
			if targetSquare >= 0 && targetSquare < 64 &&
				bb.GetPieceColor(63-targetSquare) == 1 &&
				bb.GetPieceType(63-targetSquare) != 0 {
				moves = append(moves, Move{From: square, To: square - 9})
			}
		}
	}

	return moves
}

// GetRookMoves returns all legal rook moves from a given square
func (bb *Bitboard) GetRookMoves(square int) []Move {
	moves := make([]Move, 0, 14)

	if bb.GetPieceType(square) != 4 {
		fmt.Printf("Square %d does not contain a rook\n", square)
		return moves
	}

	isWhite := bb.GetPieceColor(square) == 1
	directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}} // right, left, up, down

	for _, dir := range directions {
		for mult := 1; ; mult++ {
			targetRank := square/8 + dir[0]*mult
			targetFile := square%8 + dir[1]*mult

			if targetRank < 0 || targetRank > 7 || targetFile < 0 || targetFile > 7 {
				break
			}

			targetSquare := targetRank*8 + targetFile
			flippedTargetSquare := 63 - targetSquare

			if bb.GetOccupiedSquares()>>flippedTargetSquare&1 == 0 {
				moves = append(moves, Move{From: square, To: targetSquare})
				continue
			}

			pieceColorOnTarget := bb.GetPieceColor(targetSquare)
			if (isWhite && pieceColorOnTarget == 0) || (!isWhite && pieceColorOnTarget == 1) {
				moves = append(moves, Move{From: square, To: targetSquare})
			}
			break
		}
	}

	return moves
}

// GetAllPiecesMoves returns all legal moves for all pieces of a given type
func (bb *Bitboard) GetAllPiecesMoves(piece rune) []Move {
	moves := make([]Move, 0, 32)

	var pieceBB uint64
	switch piece {
	case 'p':
		pieceBB = bb.Wpawn
	case 'P':
		pieceBB = bb.Bpawn
	case 'r':
		pieceBB = bb.Wrook
	case 'R':
		pieceBB = bb.Brook
	default:
		return moves
	}

	for i := 0; i < 64; i++ {
		if pieceBB>>i&1 == 1 {
			square := 63 - i
			var pieceMoves []Move
			switch piece {
			case 'p', 'P':
				pieceMoves = bb.GetPawnMoves(square)
			case 'r', 'R':
				pieceMoves = bb.GetRookMoves(square)
			}
			moves = append(moves, pieceMoves...)
		}
	}

	return moves
}
