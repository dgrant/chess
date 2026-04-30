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

~~This position:
rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1
has trouble getting a next move~~

Fixed. Black is in check from Qf7+ with Kh8 the only legal move. Engine
returns Kh8 at every depth (1–6) and within any reasonable time budget.
Pinned by `chesslib/tests/check_position_regression_tests.rs`.

While investigating, found a deeper related bug: the eval used
CHECKMATE_BONUS=100000 (constant, no ply awareness) while search used
-30000+depth. The 100000 dominated, so the engine couldn't distinguish
mate-in-1 from mate-in-N — both scored 100000. Fixed by removing
CHECKMATE_BONUS from eval, moving the mate check in negamax to before
the depth=0 quiesce branch, and using a ply-aware mate score
(`-MATE_SCORE + ply`) so closer mates score larger. After the fix,
white correctly plays Qh7# or Qf8# at any depth ≥ 1.

~~This position:
rn2k2r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 b kq - 0 1
Engine thinks best next move is: e8f8 which makes no sense, why is it not choosing to castle?~~

Fixed by quiescence search + MVV-LVA + the PST revert. At depth 4 the engine now
scores O-O at 125 (tied for best) and Kf8 at 205 (~80cp worse). Pinned by
`chesslib/tests/castling_regression_tests.rs`.