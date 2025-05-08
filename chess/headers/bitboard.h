typedef struct bitboard
{
    unsigned long long Wpawn;
    unsigned long long Wknight;
    unsigned long long Wbishop;
    unsigned long long Wrook;
    unsigned long long Wqueen;
    unsigned long long Wking;

    unsigned long long Bpawn;
    unsigned long long Bknight;
    unsigned long long Bbishop;
    unsigned long long Brook;
    unsigned long long Bqueen;
    unsigned long long Bking;
    
} Bitboard;

void init_bitboard(Bitboard *bitboard);

char *bitboard_bitstr(Bitboard *bitboard, int new_lines);
char *bitboard_str(Bitboard *bitboard);
char *bitboard_FEN(Bitboard *bitboard);

void FEN_to_bitboard(char FEN[], Bitboard *bitboard);

unsigned long long get_occupied_squares(Bitboard *bitboard);
unsigned long long get_empty_squares(Bitboard *bitboard);
unsigned long long get_white_squares(Bitboard *bitboard);
unsigned long long get_black_squares(Bitboard *bitboard);

int get_num_nonzero_moves(unsigned int *moves);

void clear_bitboard(Bitboard *bitboard);
