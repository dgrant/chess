# TODO

To handle king in check
wen we generate possible moves, if the king is in check, we must only consider moves which bring the king out of check.
I assume a simple way to do this, is to try making the move, then check the board's state to see if the king is in check.

v0.1 - still random moves, just correctly generates moves for all pieces
king moves - DONE (king-walks-into-check is filtered by is_legal_move).
pawn promotion - DONE.
castling - DONE.
en passant - DONE (move generation works; FEN round-trip also done).
proper fen generation of move numbers - DONE (halfmove + fullmove counters
   tracked through apply_move/undo_last_move and round-tripped via load_fen
   and to_fen). Pinned by chesslib/tests/fen_roundtrip_tests.rs.
perft for accuracy - DONE (chesslib/tests/perft_tests.rs).

v0.2 - actually evaluate positions - DONE (chesslib/src/evaluation.rs).

v0.3 - actually descend a tree, maybe just very shallow, like 2 or 3 moves deep
   DONE — negamax + alpha-beta + iterative deepening (caps at depth 20),
   quiescence at the depth-0 horizon, MVV-LVA capture ordering, killer
   moves + history heuristic for quiet-move ordering.

Better evaluation - PARTIAL. Material, piece-square tables (PeSTO),
mobility (currently disabled), king safety / castling, bishop pair, check
bonus. Open: pawn structure (was tried + reverted), passed-pawn bonuses,
king-tropism, transposition table (the next likely perf step per
ARCHITECTURE.md).

# Features

Hint support — engine returns its preferred move for the side to move
*without* requiring `go` semantics or moving the game forward. Use cases:
GUI "show hint" button for human players, training tools, analysis mode.

Open design questions:
  - Protocol surface: xboard/winboard has a literal `hint` command;
    UCI doesn't. We could (a) add an xboard mode, (b) extend our UCI
    handler with a non-standard `hint` command, or (c) expose hint
    via a separate non-UCI library entry point.
  - Depth/time policy: a hint is typically expected to be cheap —
    sub-second, not a full search. Probably "go to depth 6" or
    "search 200ms" rather than full time-control rules.
  - Should the hint search reuse the session's transposition table
    (when we have one) so a "hint then go" sequence is fast?

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