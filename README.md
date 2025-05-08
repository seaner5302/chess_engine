# chess_engine (in progess)
### Goal:
Create a fast move generator for the game of Chess using manual memory management in C.
### Setup:
```
git clone git@github.com:seaner5302/chess_engine.git
cd chess_engine/chess
make
```
### Executing Tests:
After "make" step in Setup you can execute tests.
```
./test_pawn_moves.x
./test_rook_moves.x
```
Each test case should print the following textual ouput:\
![image](https://github.com/user-attachments/assets/e7a3a60b-1cad-495c-a724-b826023b0591)\
## Interpreting Results:
Results are expressed in algebraic notation with white pieces being lowercase and black pieces being uppercase.\See https://en.wikipedia.org/wiki/Algebraic_notation_(chess) for more info about that.\
Here is a Chess board for reference!\
<img src="https://github.com/user-attachments/assets/40a0163b-7b64-41a8-8230-ed380f165a0c" width="400" height="400">\
### Suite Results:
After testing is finished the following summary will be printed:\
![image](https://github.com/user-attachments/assets/bc46562e-7c27-4568-96b2-d0bc68612626)\
Shoutout to https://www.chessprogramming.org/Main_Page and [lichess.org](https://lichess.org/) for providing guidance, both great places to learn more. I'm hoping to add more pieces shortly, exhaustive testing has been more extensive than I had planned :( .
