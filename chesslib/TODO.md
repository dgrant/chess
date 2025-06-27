# TODO

To handle king in check
wen we generate possible moves, if the king is in check, we must only consider moves which bring the king out of check.
I assume a simple way to do this, is to try making the move, then check the board's state to see if the king is in check.

v0.1 - still random moves, just correctly generates moves for all pieces
king moves - just had a case where a king move was generated that would have put the king in check, I think I didn't handle king attacking squares.
pawn promotion - done
castling - done
en passant - todo
proper fen generation of move numbers
perft for accuracy

v0.2 - actually evaluate positions
compare positions maybe only based on piece values initially, and only return the best move

v0.3 - actually descend a tree, maybe just very shallow, like 2 or 3 moves deep

Maybe better evaluation? Based on more than just material, maybe positional evaluation, like how many squares a piece controls, how many pieces are attacking a square, etc.

# Bugs

This position:
rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1
has trouble getting a next move

This position:
rn2k2r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 b kq - 0 1
Engine thinks best next move is: e8f8 which makes no sense, why is it not choosing to castle?