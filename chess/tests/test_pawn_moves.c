#define MAIN
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>
#include <pthread.h>
#include "../headers/bitboard.h"
#include "../headers/bb_moves.h"

int print_results(char *FEN, int sq, char test_name[], 
                  unsigned int expected_moves[2], unsigned int verbose) {
    Bitboard bitboard;
    unsigned int *pawn_moves;
    char *bb_str;
    int valid = 1;
    char algebraic[2];
    FEN_to_bitboard(FEN, &bitboard);
    pawn_moves = get_pawn_moves(&bitboard, sq);
    bb_str = bitboard_str(&bitboard);
    
    if (verbose) {
        printf("Test Name: %s\n", test_name);
        printf("%s\n", bb_str);
        numeric_to_algebraic(sq, algebraic);
        printf("Getting moves for pawn on %s\n[", algebraic);
    }

    for (int i= 0; i <= get_num_nonzero_moves(pawn_moves); i++) {
        if (verbose) {
            printf("%d ",pawn_moves[i]);
            numeric_to_algebraic(pawn_moves[i], algebraic);
            printf("%s", algebraic);
            if (i != get_num_nonzero_moves(pawn_moves)) {
                printf(", ");
            }
        }
        if (pawn_moves[i] != expected_moves[i]){
            valid = 0;
        }
    }
    if (verbose) {
        printf("]");
        if (valid) printf("\nVALID"); else printf("\nINVALID");
        printf("\n------------------------------------------------------------\n");
    }
    free(pawn_moves); free(bb_str);
    return valid;
}

void run_all_tests(char test_names[12][30], unsigned int squares[12], 
                   char board_states[12][80], unsigned int expected_moves[12][2],
                   unsigned int verbose) {
    unsigned int validity[12] = {1,1,1,1,1,1,1,1,1,1,1,1};
    unsigned int successes = 0;
    unsigned int failures = 0;
    unsigned int num_tests = 12;
    if (!verbose) printf("------------------------------------------------------------");
    printf("\n");
    for (int bs = 0; bs < num_tests; bs++) {
        char state_name[200] = "Board state: ";
        strcat(state_name, test_names[bs]);
        validity[bs] = print_results(board_states[bs],squares[bs], state_name, expected_moves[bs], verbose);
    }
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
    unsigned int expected_moves[12][2] = {{20,28},{20,0},{28,0},{0,0},{19,21},{23,0},
                                          {44, 36}, {44, 0}, {36, 0}, {0, 0}, {45,43}, {47, 0}};
    unsigned int squares[12] = {12, 12, 20, 12, 12, 16,
                                52, 52, 44, 52, 52, 56};
    // Bitboard bitboard;
    // unsigned int *moves;
    char test_names[12][30] = {"WP push 2, no captures ",
                               "WP push 1, no captures ",
                               "WP push 1, no captures ",
                               "WP no push, no captures",
                               "WP no push, 2 captures ",
                               "WP no push, 1 captures ",
                               "BP push 2, no captures ",
                               "BP push 1, no captures ",
                               "BP push 1, no captures ",
                               "BP no push, no captures",
                               "BP no push, 2 captures ",
                               "BP no push, 1 captures ",
                               };
    char board_states[12][80] = {"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/3p4/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/8/3p4/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/3p4/3p4/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/3p4/2PPP3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/7p/PPPPPPPP/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/3p4/8/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/8/3P4/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/8/3P4/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/8/2ppp3/PPPPPPPP/RNBQKBNR w KQkq 0 0",
                                 "rnbqkbnr/pppppppp/8/8/8/6pp/7P/RNBQKBNR w KQkq 0 0"
                                };
    struct timeval beg_time, end_time;
    clock_t begin, end;
    begin = clock();
    gettimeofday(&beg_time, NULL);

    run_all_tests(test_names, squares, board_states, expected_moves, 1);
    
    end = clock();
    double cpu_time = ((double) (end - begin)) / CLOCKS_PER_SEC;
    gettimeofday(&end_time, NULL);
    double wall_time = 
    ((end_time.tv_sec-beg_time.tv_sec)) + 
    ((end_time.tv_usec-beg_time.tv_usec) / 1000000.0);
    printf("wall time: %10.4e sec\ncpu time: %10.4e sec\n", wall_time, cpu_time);

    // wall time: 1.1370e-03 sec
    // cpu time: 1.1410e-03 sec

    // FEN_to_bitboard("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 0", &bitboard);

    // moves = get_all_pieces_moves(&bitboard, 'P');

    // for (int i= 0; i <= 31; i++) {
    //         if (moves[i] != 0){
    //             //printf("%d ",moves[i]);
    //             char algebraic[2];
    //             numeric_to_algebraic(moves[i], algebraic);
    //             //printf("%c%c\n",algebraic[0], algebraic[1]);
    //         }
    // }

    // free(moves);

    return 0;
}