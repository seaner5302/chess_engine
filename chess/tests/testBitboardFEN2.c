#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../headers/bitboard.h"
#define MAIN

int main(void) {
    Bitboard *bitboard;
    char FEN[100] = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RBNQKNBR w KQkq 0 0";
    FEN_to_bitboard(FEN, bitboard);
    printf("%s",bitboard_bitstr(bitboard, 0));
    return 0;
}