package chess

import (
	"strings"
)

// ToFEN converts the bitboard to a FEN string representation
func (bb *Bitboard) ToFEN() string {
	var builder strings.Builder
	emptyCount := 0

	// Process each rank from 8 to 1 (top to bottom)
	for rank := 7; rank >= 0; rank-- {
		for file := 0; file < 8; file++ {
			square := rank*8 + file
			pieceType := bb.GetPieceType(square)
			pieceColor := bb.GetPieceColor(square)

			if pieceType == 0 {
				emptyCount++
			} else {
				if emptyCount > 0 {
					builder.WriteString(string(rune('0' + emptyCount)))
					emptyCount = 0
				}

				var piece rune
				switch pieceType {
				case 1: // Pawn
					piece = 'P'
				case 2: // Knight
					piece = 'N'
				case 3: // Bishop
					piece = 'B'
				case 4: // Rook
					piece = 'R'
				case 5: // Queen
					piece = 'Q'
				case 6: // King
					piece = 'K'
				}

				if pieceColor == 0 { // Black
					piece = rune(strings.ToLower(string(piece))[0])
				}

				builder.WriteRune(piece)
			}
		}

		if emptyCount > 0 {
			builder.WriteString(string(rune('0' + emptyCount)))
			emptyCount = 0
		}

		if rank > 0 {
			builder.WriteRune('/')
		}
	}

	// Add other FEN fields (active color, castling, en passant, etc.)
	// For now, we'll just add placeholders
	builder.WriteString(" w KQkq - 0 1")

	return builder.String()
}

// FromFEN sets the bitboard state from a FEN string
func (bb *Bitboard) FromFEN(fen string) error {
	// Clear the current board state
	bb.ClearBitboard()

	// Split FEN into its components (we'll only use the piece placement for now)
	fields := strings.Fields(fen)
	if len(fields) == 0 {
		return nil
	}

	ranks := strings.Split(fields[0], "/")
	if len(ranks) != 8 {
		return nil
	}

	// Process each rank
	for rankIdx, piecesStr := range ranks {
		file := 0
		for _, pieceChar := range piecesStr {
			if pieceChar >= '1' && pieceChar <= '8' {
				file += int(pieceChar - '0')
				continue
			}

			if file >= 8 {
				continue
			}

			square := (7-rankIdx)*8 + file
			flippedSquare := 63 - square

			switch pieceChar {
			case 'P':
				bb.Wpawn |= 1 << flippedSquare
			case 'N':
				bb.Wknight |= 1 << flippedSquare
			case 'B':
				bb.Wbishop |= 1 << flippedSquare
			case 'R':
				bb.Wrook |= 1 << flippedSquare
			case 'Q':
				bb.Wqueen |= 1 << flippedSquare
			case 'K':
				bb.Wking |= 1 << flippedSquare
			case 'p':
				bb.Bpawn |= 1 << flippedSquare
			case 'n':
				bb.Bknight |= 1 << flippedSquare
			case 'b':
				bb.Bbishop |= 1 << flippedSquare
			case 'r':
				bb.Brook |= 1 << flippedSquare
			case 'q':
				bb.Bqueen |= 1 << flippedSquare
			case 'k':
				bb.Bking |= 1 << flippedSquare
			}
			file++
		}
	}

	return nil
}
