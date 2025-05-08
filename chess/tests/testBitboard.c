#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../headers/bitboard.h"
#define MAIN

int main(void) {
    Bitboard bitboard;
    init_bitboard(&bitboard);
    int curbit;
    printf("\nBpawn Bitboard:");
    for (int i = 0; i < 64; i++){
        if (i % 8 == 0){
            printf("\n");
        }
        curbit = (bitboard.Bpawn >> i) & 1;
        printf("%d ", curbit);
    }
    printf("\n");
    printf("\n");
    return 0;
}

