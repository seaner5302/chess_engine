# chess_engine (in progress)
### Goal
Create a fast move generator for the game of chess using manual memory management in C.
Implement chess logic using bitboards for board representation (https://www.chessprogramming.org/Bitboards).
### Setup
To compile this C code you will need a C compiler (GCC).
If you don't have GCC already, refer to this guide https://gcc.gnu.org/install/. \
Once you have GCC, you can run the following commands to install and build the project.
```
git clone git@github.com:seaner5302/chess_engine.git
cd chess_engine/chess
make
```
### Executing Tests
After the "make" step in Setup above, you can execute tests.
```
./test_pawn_moves.x
```
Each test case should print the following textual ouput:\
\
![image](https://github.com/user-attachments/assets/8ca3feae-5807-41ff-9025-80ee3b9de9fc)
## Interpreting Results
Results are expressed in algebraic notation with white pieces being lowercase and black pieces being uppercase. See https://en.wikipedia.org/wiki/Algebraic_notation_(chess) for more info about that.\
Here is a chess board for comparison:\
\
<img src="https://github.com/user-attachments/assets/40a0163b-7b64-41a8-8230-ed380f165a0c" width="400" height="400"> <img src="https://github.com/user-attachments/assets/974d1f66-0493-4a84-a701-b33c3ec0352b" width="400" height="400">

### Suite Results
After testing is finished the following summary will be printed:\
\
![image](https://github.com/user-attachments/assets/2ba27da1-4ec1-4ed8-a60f-24159c3f5d81)
\
Shoutout to https://www.chessprogramming.org/Main_Page and [lichess.org](https://lichess.org/) for providing guidance, they are both great places to learn more if you want to start your own chess programming project. I'm hoping to add more pieces shortly, exhaustive testing has been more extensive than I had planned... to be continued.
