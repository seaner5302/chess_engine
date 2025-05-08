#define MAIN
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../headers/bitboard.h"
#include "../headers/bb_moves.h"

int print_results(char *FEN, int sq, char test_name[], 
                  unsigned int expected_moves[2], unsigned int verbose) {
    Bitboard bitboard;
    unsigned int *rook_moves;
    char *bb_str;
    int valid = 1;
    char algebraic[2];
    FEN_to_bitboard(FEN, &bitboard);
    rook_moves = get_rook_moves(&bitboard, sq);
    bb_str = bitboard_str(&bitboard);
    
    if (verbose) {
        printf("Test Name: %s\n", test_name);
        printf("%s\n", bb_str);
        numeric_to_algebraic(sq, algebraic);
        printf("Getting moves for rook on %s\n[", algebraic);
    }
    for (int i= 0; i <= get_num_nonzero_moves(rook_moves); i++) {
        if (verbose) {
            printf("%d ",rook_moves[i]);
            numeric_to_algebraic(rook_moves[i], algebraic);
            printf("%s", algebraic);
            if (i != get_num_nonzero_moves(rook_moves)) {
                printf(", ");
            }
        }
        if (rook_moves[i] != expected_moves[i]){
            valid = 0;
        }
    }
    if (verbose) {
        printf("]");
        if (valid) printf("\nVALID");
        if (!valid) printf("\nINVALID");
        printf("\n------------------------------------------------------------");
    }
    free(rook_moves);
    free(bb_str);
    return valid;
}

void run_all_tests(char test_names[2][30], unsigned int squares[2], 
                   char board_states[2][80], unsigned int expected_moves[2][28],
                   unsigned int verbose) {
    unsigned int validity[2];
    unsigned int successes = 0;
    unsigned int failures = 0;
    unsigned int num_tests = 2;
    for (int bs = 0; bs < num_tests; bs++) {
        char state_name[200] = "Board state ";
        char bsc[2];
        char numchar = (char)(bs+1);
        bsc[0] = numchar;
        bsc[1] = '\0';
        strcat(state_name, bsc);
        strcat(state_name, ": ");
        strcat(state_name, test_names[bs]);
        validity[bs] = print_results(board_states[bs],squares[bs], 
                                     state_name, expected_moves[bs], verbose);
    }
    if (!verbose) printf("------------------------------------------------------------");
    printf("\n");
    for (int bs = 0; bs < num_tests; bs++) {
        int needs_space = (bs+1) / 10;
        char *sp = "";
        if (needs_space == 0) sp = " ";
        if (validity[bs]) {
            printf("| Test Number: %c%d | Name: %s | PASSED |\n",*sp, bs+1, test_names[bs]);
            successes++;
        }
        else {
            printf("| Test Number: %c%d | Name: %s | FAILED |\n", *sp, bs+1, test_names[bs]);
            failures++; 
        }
    }
    printf("------------------------------------------------------------\n");
    printf("TEST SUMMARY\n");
    printf("Passed %d/%d\n", successes, successes+failures);
}

int main(void) {
    unsigned int expected_moves[2][28] = {{2,3,4,5,6,7,8,9,17,25,33,41,49,57},
                                           {}};
    unsigned int squares[2] = {1, 1};
    char test_names[2][30] = {"WR slide max",
                              "WR no slide"};
    char board_states[2][80] = {"7r/8/8/8/8/8/8/8 w KQkq 0 0",
                                "6pr/7p/8/8/8/8/8/8 w KQkq 0 0"};
    run_all_tests(test_names, squares, board_states, expected_moves, 1);

    return 0;
}