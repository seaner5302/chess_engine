#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../headers/bitboard.h"
#define MAIN

int main(void) {
    Bitboard *bitboard;
    init_bitboard(bitboard);
    printf("%s", bitboard_bitstr(bitboard, 1));
    printf("\n");
    return 0;
}