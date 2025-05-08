#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../headers/bitboard.h"
#define MAIN

int main(void) {
    Bitboard bitboard;
    int str_index = 0;
    // unsigned long long occupied_bb;
    init_bitboard(&bitboard);
    char *bitboard_str = bitboard_FEN(&bitboard);
    unsigned long long occupied_bb = get_empty_squares(&bitboard);
    printf("%llu\n\n", get_occupied_squares(&bitboard));
    for (int i = 0; i < 64; i++)
        {
           bitboard_str[str_index++] = (char)((occupied_bb >> i & 1) + 48);
        if ((str_index != 0 && (str_index+1) % 9 == 0) | i == 63)
        {
             bitboard_str[str_index++] = '\n';
        }
    }
    printf("%s\n", bitboard_str);
    return 0;
}