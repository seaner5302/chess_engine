#include <stdlib.h>
#include <stdio.h>
#ifdef MAIN
#else
    #include "headers/bb_moves.h"
#endif

// take in numeric square int 1-64 and convert to
// algebraic notation string length 2 "e4"
void numeric_to_algebraic(unsigned int numeric, char *algebraic) {
    if (numeric < 1) {
        algebraic[0]= ' ';
        algebraic[1]= ' ';
        return;
    }
    unsigned int rank= (numeric-1) / 8;
    unsigned int file= (numeric-1) % 8;
    algebraic[0]= 'a' + file;
    algebraic[1]= '1' + rank;
}

// take in algebraic notation string and convert to
// numeric square int 1-64
int algebraic_to_numeric(char *alge) {
    // subtract char is position in ASCII for rank and file
    return (alge[0]-96) + (alge[1]-49) * 8;
}

int get_piece_type(Bitboard *board, int square) {
    if ((board->Wpawn >> square & 1) || (board->Bpawn >> square & 1)) return 1;
    if ((board->Wknight >> square & 1) || (board->Bknight >> square & 1)) return 2;
    if ((board->Wbishop >> square & 1) || (board->Bbishop >> square & 1)) return 3;
    if ((board->Wrook >> square & 1) || (board->Brook >> square & 1)) return 4;
    if ((board->Wqueen >> square & 1) || (board->Bqueen >> square & 1)) return 5;
    if ((board->Wking >> square & 1) || (board->Bking >> square & 1)) return 6;
    return 0;
}

int get_piece_color(Bitboard *board, int square) {
    if ((board->Wpawn >> square & 1) || (board->Wknight >> square & 1) ||
    (board->Wbishop >> square & 1) || (board->Wrook >> square & 1) ||
    (board->Wqueen >> square & 1) || (board->Wking >> square & 1)) {
        return 1;
    }

    if ((board->Bpawn >> square & 1) || (board->Bknight >> square & 1) ||
    (board->Bbishop >> square & 1) || (board->Brook >> square & 1) ||
    (board->Bqueen >> square & 1) || (board->Bking >> square & 1)) {
        return 0;
    }
    return -1;
}

unsigned int *get_pawn_moves(Bitboard *board, int square) {
    unsigned int *moves = calloc(4, 4);
    int move_index= 0;
    int flipped_square = 63-square`;
    if (get_piece_type(board, flipped_square) != 1) {
        printf("%d\n",get_piece_type(board, flipped_square));
        printf("Square selected does not contain a pawn %d\n", square);
        return moves;
    }
    if (get_piece_color(board, flipped_square)) {
        if (!(get_occupied_squares(board) >> (flipped_square-8) & 1)) {
            moves[move_index++] = square+8;
            if (square >= 8 && square <= 16 && !(get_occupied_squares(board) >> (flipped_square-16) & 1)) {
                moves[move_index++] = square+16;
            }
        }
        // check if capture is possible
        if (!get_piece_color(board, flipped_square-7)) {
            moves[move_index++] = square+7;
        }
        if (!get_piece_color(board, flipped_square-9)) {
            moves[move_index++] = square+9;
        }
    }
    else {
        if (!(get_occupied_squares(board) >> (flipped_square-8) & 1)) {
            moves[move_index++] = square-8;
            if (square >= 48 && square <= 56 && !(get_occupied_squares(board) >> (flipped_square-16) & 1)) {
                moves[move_index++] = square-16;
            }
        }
        // check if capture is possible
        if ((square+1) % 8 != 1 && get_piece_color(board, flipped_square+7) == 1) {
            moves[move_index++] = square-7;
        }
        if ((square+1) % 8 != 0 && get_piece_color(board, flipped_square+9) == 1) {
            moves[move_index++] = square-9;
        }
    }
    return moves;
}

unsigned int *get_rook_moves(Bitboard *board, int square) {
    unsigned int *moves = calloc(28, 4);
    int move_index=0;
    int flipped_square = 64 - square;
    if (get_piece_type(board, flipped_square) != 4) {
        printf("%d\n",get_piece_type(board, flipped_square));
        printf("Square selected does not contain a rook %d\n", square);
        return moves;
    }
    if (get_piece_color(board, flipped_square)) {
        // get horizontal
        int blocked = 0;
        int slide = 1;
        // check right slide moves
        while (!blocked) {
            if ((flipped_square+slide) % 8 == 0) {
                blocked = 1;
            }
            else if (get_empty_squares(board) >> (flipped_square+slide) & 1) {
                moves[move_index++] = square+slide++;
            }
            else if (get_white_squares(board) >> (flipped_square+slide) & 1) {
                blocked = 1;
            }
            else if ((get_black_squares(board) >> (flipped_square+slide) & 1)) {
                moves[move_index++] = square+slide++;
                blocked = 1;
            }
        }
        // check left slide moves
        blocked = 0;
        slide = 1;
        while (!blocked) {
            if ((flipped_square-slide) % 8 == 0) {
                blocked = 1;
            }
            if (get_empty_squares(board) >> (flipped_square+slide) & 1) {
                moves[move_index++] = square+slide++;
            }
            else if (get_white_squares(board) >> (flipped_square+slide) & 1) {
                blocked = 1;
            }
            else if ((get_black_squares(board) >> (flipped_square+slide) & 1)) {
                moves[move_index++] = square-slide++;
                blocked = 1;
            }
        }
        // check up slide moves
        blocked = 0;
        slide = 1;
        while (!blocked) {
            if ((flipped_square+slide*8) > 63) {
                blocked = 1;
            }
            else if (get_empty_squares(board) >> (flipped_square+slide*8) & 1) {
                moves[move_index++] = square-slide*8;
                slide++;
            }
            else if (get_white_squares(board) >> (flipped_square+slide*8) & 1) {
                blocked = 1;
            }
            else if ((get_black_squares(board) >> (flipped_square+slide*8) & 1)) {
                moves[move_index++] = square-slide*8;
                slide++;
                blocked = 1;
            }
        }
        // check down slide moves
        blocked = 0;
        slide = 1;
        while (!blocked) {
            if ((flipped_square-slide) < 0) {
                blocked = 1;
            }
            if (get_empty_squares(board) >> (flipped_square-slide*8) & 1) {
                moves[move_index++] = square+slide++*8;
            }
            else if (get_white_squares(board) >> (flipped_square-slide*8) & 1) {
                blocked = 1;
            }
            else if ((get_black_squares(board) >> (flipped_square-slide*8) & 1)) {
                moves[move_index++] = square+slide++*8;
                blocked = 1;
            }
        }
    }
    return moves;
}

unsigned int *get_all_pieces_moves(Bitboard *board, char piece) {
    unsigned int *moves = calloc(32, 4);
    unsigned int *curr_moves;
    unsigned int moves_index = 0;
    if (piece == 'p') {
        for (int i = 0; i < 64; i++) {
            if ((board->Wpawn) >> i & 1) {
                curr_moves = get_pawn_moves(board, 64-i);
                for (int mi = 0; mi < 4; mi++) {
                    moves[moves_index++] = curr_moves[mi];
                }
            }
        }
    }
    else if (piece == 'P') {
        for (int i = 0; i < 64; i++) {
            if ((board->Bpawn) >> i & 1) {
                curr_moves = get_pawn_moves(board, 64-i);
                for (int mi = 0; mi < 4; mi++) {
                    moves[moves_index++] = curr_moves[mi];
                }
            }
        }
    }
    // implement else if check for all piece types here
    free(curr_moves);
    return moves;
}

int get_num_nonzero_moves(unsigned int *moves) {
    // Max number of moves a piece can have is q 27
    for (int i= 0; i <= 27; i++) {
        // 0 signifies there are no more moves left in arr
        if (moves[i] == 0) {
            return i-1;
        }
    }
    return 0;
}
