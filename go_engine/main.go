package main

import (
	"fmt"
	"github.com/sdimick33/GitYeah/go_engine/chess"
)

func main() {
	// Create a new board in the initial position
	board := chess.InitBitboard()

	// Print the initial board position
	fmt.Println("Initial board position:")
	fmt.Println(board.String())

	// Get and print all possible pawn moves for white
	fmt.Println("\nAll possible white pawn moves:")
	moves := board.GetAllPiecesMoves('p')
	for _, move := range moves {
		from := chess.NumericToAlgebraic(move.From)
		to := chess.NumericToAlgebraic(move.To)
		fmt.Printf("%s -> %s\n", from, to)
	}

	// Get and print all possible pawn moves for black
	fmt.Println("\nAll possible black pawn moves:")
	moves = board.GetAllPiecesMoves('P')
	for _, move := range moves {
		from := chess.NumericToAlgebraic(move.From)
		to := chess.NumericToAlgebraic(move.To)
		fmt.Printf("%s -> %s\n", from, to)
	}

	// Convert current position to FEN
	fmt.Println("\nFEN string of current position:")
	fmt.Println(board.ToFEN())

	// Create a new position from a FEN string
	testFEN := "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
	newBoard := &chess.Bitboard{}
	newBoard.FromFEN(testFEN)

	fmt.Println("\nBoard position from FEN string:")
	fmt.Println(newBoard.String())
} 