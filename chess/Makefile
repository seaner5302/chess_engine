CFLAGS = -Wall -Werror -g -Og
CC     = gcc $(CFLAGS)
SHELL  = /bin/bash

ALL_OBJS = \
	bb_moves.o bitboard.o \

%.o : %.c
	gcc -c $<

all : test_pawn_moves test_rook_moves

test_pawn_moves : $(ALL_OBJS) tests/test_pawn_moves.c
	$(CC) $^ -o test_pawn_moves.x

test_rook_moves : $(ALL_OBJS) tests/test_rook_moves.c
	$(CC) $^ -o test_rook_moves.x

clean : 
	rm *.o *.x