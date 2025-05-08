#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#ifdef MAIN
#else
#include "headers/bitboard.h"
#endif

char piece_names[] = {'p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'};

void init_bitboard(Bitboard *bitboard)
{
    bitboard->Wpawn = (unsigned long long)255 << 48;
    bitboard->Wknight = (unsigned long long)66 << 56; // knight pos 01000010
    bitboard->Wbishop = (unsigned long long)36 << 56;
    bitboard->Wrook = (unsigned long long)129 << 56;
    bitboard->Wqueen = (unsigned long long)8 << 56; // queen pos 00001000
    bitboard->Wking = (unsigned long long)16 << 56;

    bitboard->Bpawn = (unsigned long long)255 << 8;
    bitboard->Bknight = (unsigned long long)66;
    bitboard->Bbishop = (unsigned long long)36;
    bitboard->Brook = (unsigned long long)129;
    bitboard->Bqueen = (unsigned long long)8;
    bitboard->Bking = (unsigned long long)16;
}

void clear_bitboard(Bitboard *bitboard)
{
    bitboard->Wpawn = (unsigned long long)0;
    bitboard->Wknight = (unsigned long long)0; // knight pos 01000010
    bitboard->Wbishop = (unsigned long long)0;
    bitboard->Wrook = (unsigned long long)0;
    bitboard->Wqueen = (unsigned long long)0; // queen pos 00001000
    bitboard->Wking = (unsigned long long)0;

    bitboard->Bpawn = (unsigned long long)0;
    bitboard->Bknight = (unsigned long long)0;
    bitboard->Bbishop = (unsigned long long)0;
    bitboard->Brook = (unsigned long long)0;
    bitboard->Bqueen = (unsigned long long)0;
    bitboard->Bking = (unsigned long long)0;
}

char *bitboard_bitstr(Bitboard *bitboard, int new_lines)
{   
    char *bitboard_str = calloc(128 * 12, 1);
    int str_index = 0;
    char curr_piece;

    for (int num_board = 0; num_board < 12; num_board++)
    {
        curr_piece = piece_names[num_board];
        bitboard_str[str_index++] = piece_names[num_board];
        bitboard_str[str_index++] = '\n';
        if (curr_piece == 'p')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wpawn) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece == 'n')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wknight) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece == 'b')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wbishop) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece =='r')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wrook) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece == 'q')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wqueen) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }

        else if (curr_piece == 'k')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Wking) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }

        else if (curr_piece == 'P')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Bpawn) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }

        else if (curr_piece == 'N')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Bknight) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece == 'B')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Bbishop) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }

        else if (curr_piece == 'R')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Brook) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
        else if (curr_piece == 'Q')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Bqueen) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }

        else if (curr_piece == 'K')
        {
            for (int i = 0; i < 64; i++)
            {
                bitboard_str[str_index++] = (char)(((bitboard->Bking) >> i & 1) + 48);
                if ((str_index != 0 && (str_index - 2 - 3 * num_board) % 9 == 0 && new_lines) | i == 63)
                {
                    bitboard_str[str_index++] = '\n';
                }
            }
        }
    }
    return bitboard_str;
}

char *bitboard_str(Bitboard *bitboard) {
    char *board_str = calloc(72, 1);
    int curr_piece_index;
    char curr_piece;
    unsigned long long curr_bb;

    for (int i = 0; i < 71; i++) {
        board_str[i] = '-';
        if ((i+1) % 9 == 0) {
            board_str[i] = '\n';
        }
    }

    for (int num_board = 0; num_board < 12; num_board++)
    {
        curr_piece = piece_names[num_board];
        if (curr_piece == 'p') curr_bb = bitboard->Wpawn;
        else if (curr_piece == 'n') curr_bb = bitboard->Wknight;
        else if (curr_piece == 'b') curr_bb = bitboard->Wbishop;
        else if (curr_piece == 'r') curr_bb = bitboard->Wrook;
        else if (curr_piece == 'q') curr_bb = bitboard->Wqueen;
        else if (curr_piece == 'k') curr_bb = bitboard->Wking;
        else if (curr_piece == 'P') curr_bb = bitboard->Bpawn;
        else if (curr_piece == 'N') curr_bb = bitboard->Bknight;
        else if (curr_piece == 'B') curr_bb = bitboard->Bbishop;
        else if (curr_piece == 'R') curr_bb = bitboard->Brook;
        else if (curr_piece == 'Q') curr_bb = bitboard->Bqueen;
        else if (curr_piece == 'K') curr_bb = bitboard->Bking;
        else curr_bb = 0;

        for (int i = 0; i < 64; i++) {
            char curr_square = (curr_bb >> i & 1);
            if (curr_square) {
                board_str[i+i/8] = curr_piece;
            }
        }
    }
    return board_str;
}

char *bitboard_FEN(Bitboard *bitboard)
{
    char *board_str = calloc(85, 1);
    int index;
    int bb_index;
    int square_count;
    int empty_count;
    unsigned long long curr_square;
    unsigned long long w_squares = get_white_squares(bitboard);
    unsigned long long b_squares = get_black_squares(bitboard);
    unsigned long long occ_squares = get_occupied_squares(bitboard);

    for (index = 0; square_count < 64; index++)
    {
        curr_square = occ_squares >> square_count & 1;
        if (square_count % 8 == 0 && square_count != 0)
        {
            board_str[index++] = '/';
        }
        if (!curr_square)
        {
            if (!(occ_squares >> (square_count + 1) & 1) && (square_count + 1) % 8 != 0)
            {
                empty_count++;
                index--;
            }
            else
            {
                empty_count++;
                board_str[index] = '0' + empty_count;
                empty_count = 0;
            }
        }
        else
        {
            if (w_squares >> square_count & 1)
            {
                if (bitboard->Wpawn >> square_count & 1)
                {
                    board_str[index] = 'p';
                }
                else if (bitboard->Wknight >> square_count & 1)
                {
                    board_str[index] = 'n';
                }
                else if (bitboard->Wbishop >> square_count & 1)
                {
                    board_str[index] = 'b';
                }
                else if (bitboard->Wrook >> square_count & 1)
                {
                    board_str[index] = 'r';
                }
                else if (bitboard->Wqueen >> square_count & 1)
                {
                    board_str[index] = 'q';
                }
                else
                {
                    board_str[index] = 'k';
                }
            }
            else
            {
                if (bitboard->Bpawn >> square_count & 1)
                {
                    board_str[index] = 'P';
                }
                else if (bitboard->Bknight >> square_count & 1)
                {
                    board_str[index] = 'N';
                }
                else if (bitboard->Bbishop >> square_count & 1)
                {
                    board_str[index] = 'B';
                }
                else if (bitboard->Brook >> square_count & 1)
                {
                    board_str[index] = 'R';
                }
                else if (bitboard->Bqueen >> square_count & 1)
                {
                    board_str[index] = 'Q';
                }
                else
                {
                    board_str[index] = 'K';
                }
            }
        }
        square_count++;
    }
    return board_str;
}

void FEN_to_bitboard(char FEN[], Bitboard *bitboard) {
    char *FEN_cpy = calloc(80, 1);
    char *rank_str;
    int file;
    int rank = 0;
    int blank_total;
    char curr_char;
    unsigned long long curr_square;
    int file_index;
    int str_index;
    clear_bitboard(bitboard);
    strcpy(FEN_cpy, FEN);
    rank_str = strtok(FEN_cpy, "/");
    while (rank_str != 0)
    {
        file = 0;
        blank_total = 0;
        file_index = 0;
        while (file < 8)
        {
            curr_char = rank_str[file_index];
            curr_square = rank * 8 + 8 - (file)%8-1;
            // Using ASCII to see if square is number/blank
            if (curr_char <= 57 && curr_char >= 49)
            {
                blank_total = curr_char - 48;
                file += blank_total;
                file_index++;
            }
            else
            {
                // 112 -> 'p'
                if (curr_char == 112)
                {
                    bitboard->Wpawn = bitboard->Wpawn | (unsigned long long)1 << (63-curr_square);
                }
                // 80 -> 'P'
                else if (curr_char == 80)
                {
                    bitboard->Bpawn = bitboard->Bpawn | (unsigned long long)1 << (63-curr_square);
                }
                // 110 -> 'n'
                else if (curr_char == 110)
                {
                    bitboard->Wknight = bitboard->Wknight | (unsigned long long)1 << (63-curr_square);
                }
                // 78 -> 'N'
                else if (curr_char == 78)
                {
                    bitboard->Bknight = bitboard->Bknight | (unsigned long long)1 << (63-curr_square);                
                }
                // 98 -> 'b'
                else if (curr_char == 98)
                {
                     bitboard->Wbishop = bitboard->Wbishop | (unsigned long long)1 << (63-curr_square);
                }
                // 66 -> 'B'
                else if (curr_char == 66)
                {
                     bitboard->Bbishop = bitboard->Bbishop | (unsigned long long)1 << (63-curr_square);
                }
                // 114 -> 'r'
                else if (curr_char == 114)
                {
                     bitboard->Wrook = bitboard->Wrook | (unsigned long long)1 << (63-curr_square);
                }
                // 82 -> 'R'
                else if (curr_char == 82)
                {
                     bitboard->Brook = bitboard->Brook | (unsigned long long)1 << (63-curr_square);
                }
                // 113 -> 'q'
                else if (curr_char == 113)
                {
                     bitboard->Wqueen = bitboard->Wqueen | (unsigned long long)1 << (63-curr_square);
                }
                // 81 -> 'Q'
                else if (curr_char == 81)
                {
                     bitboard->Bqueen = bitboard->Bqueen | (unsigned long long)1 << (63-curr_square);
                }
                // 107 -> 'k'
                else if (curr_char == 107)
                {
                     bitboard->Wking = bitboard->Wking | (unsigned long long)1 << (63-curr_square);
                }
                // 75 -> 'K'
                else if (curr_char == 75)
                {
                     bitboard->Bking = bitboard->Bking | (unsigned long long)1 << (63-curr_square);
                }
                file_index++;
                file++;
            }
        }
        //printf("\n%s\n%llu\n", rank_str, (unsigned long long)1 << (63-curr_square));
        rank++;
        rank_str = strtok(NULL, "/");
    }
    free(FEN_cpy);
}

unsigned long long get_occupied_squares(Bitboard *bitboard)
{
    unsigned long long curr_occupied = 0;
    curr_occupied = bitboard->Wpawn | bitboard->Wknight | bitboard->Wbishop | bitboard->Wrook | bitboard->Wqueen | bitboard->Wking | bitboard->Bpawn | bitboard->Bknight | bitboard->Bbishop | bitboard->Brook | bitboard->Bqueen | bitboard->Bking;
    return curr_occupied;
}

unsigned long long get_empty_squares(Bitboard *bitboard)
{
    return ~get_occupied_squares(bitboard);
}

unsigned long long get_white_squares(Bitboard *bitboard)
{
    unsigned long long curr_occupied = 0;
    curr_occupied = bitboard->Wpawn | bitboard->Wknight | bitboard->Wbishop | bitboard->Wrook | bitboard->Wqueen | bitboard->Wking;
    return curr_occupied;
}

unsigned long long get_black_squares(Bitboard *bitboard)
{
    unsigned long long curr_occupied = 0;
    curr_occupied = bitboard->Bpawn | bitboard->Bknight | bitboard->Bbishop | bitboard->Brook | bitboard->Bqueen | bitboard->Bking;
    return curr_occupied;
}

