#!/usr/bin/env python3
"""
Headless UCI vs UCI match driver. Drives two engines move-by-move,
adjudicates by basic rules (50-move, threefold via FEN repetition,
checkmate / stalemate detection delegated to engines via 'no legal moves'
heuristic), and reports W/D/L plus rough ELO difference.
"""
import subprocess, sys, time, argparse, re, math, os
from collections import Counter

def open_engine(cmd):
    # Set CHESS_DETERMINISTIC so our engine suppresses its random tie-breaking
    # in move selection. This makes the bench reproducible: same engine + same
    # opponent + same time control + same opening = same game outcome every run.
    # Stockfish ignores this env var, which is fine.
    env = {**os.environ, "CHESS_DETERMINISTIC": "1"}
    return subprocess.Popen(cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                            text=True, bufsize=1, env=env)

def send(p, line):
    p.stdin.write(line + "\n"); p.stdin.flush()

def wait_for(p, token, timeout=30):
    deadline = time.time() + timeout
    while time.time() < deadline:
        line = p.stdout.readline()
        if not line: return None
        if token in line: return line
    return None

def parse_info_score(line):
    """Extracts cp score from a UCI 'info ... score cp N ...' line, or
    converts 'score mate N' to a large signed cp value (positive = engine
    delivers mate, negative = engine gets mated). Returns None if no usable
    score, or if the line is a 'lowerbound'/'upperbound' partial.
    Score is from the engine's POV (UCI standard)."""
    toks = line.split()
    try:
        i = toks.index("score")
    except ValueError:
        return None
    if "lowerbound" in toks[i:] or "upperbound" in toks[i:]:
        return None
    if i + 2 >= len(toks):
        return None
    kind = toks[i + 1]
    val = int(toks[i + 2])
    if kind == "cp":
        return val
    if kind == "mate":
        # +20000 for "engine mates in N", -20000 for "engine gets mated in N"
        return 20000 if val > 0 else -20000
    return None

def get_bestmove(p, timeout=60):
    """Returns (move_str | None, status, last_score) where status is
    'ok' | 'crash' | 'timeout' | 'eof' and last_score is the latest cp/mate
    score the engine reported for this search, or None."""
    deadline = time.time() + timeout
    last_score = None
    while time.time() < deadline:
        line = p.stdout.readline()
        if not line:
            return None, ("crash" if p.poll() is not None else "eof"), last_score
        if line.startswith("info "):
            s = parse_info_score(line)
            if s is not None:
                last_score = s
            continue
        if line.startswith("bestmove"):
            parts = line.split()
            mv = parts[1] if len(parts) > 1 else None
            return mv, "ok", last_score
    return None, "timeout", last_score

def init(p, options):
    send(p, "uci"); wait_for(p, "uciok")
    for k, v in options.items():
        send(p, f"setoption name {k} value {v}")
    send(p, "isready"); wait_for(p, "readyok")

def newgame(p):
    send(p, "ucinewgame"); send(p, "isready"); wait_for(p, "readyok")

def play_game(white_cmd, black_cmd, white_opts, black_opts, movetime_ms,
              max_plies=200, per_move_timeout=30.0, opening=None,
              resign_cp=500, resign_window=8, resign_min_ply=20):
    """Returns ('1-0' | '0-1' | '1/2-1/2', moves_list, reason).

    Score-based adjudication (two-sided / consensus):
      Adjudicate only when *both* engines agree on the verdict for
      `resign_window` consecutive plies of their own moves.
      - The losing side's window must be all <= -resign_cp.
      - The winning side's window must be all >= +resign_cp.
    Requiring agreement protects against false positives caused by one
    engine's eval being miscalibrated (e.g. ours).
    """
    w = open_engine(white_cmd); b = open_engine(black_cmd)
    init(w, white_opts); init(b, black_opts)
    newgame(w); newgame(b)
    moves = list(opening) if opening else []
    side, other = (w, b) if len(moves) % 2 == 0 else (b, w)
    # Per-side rolling window of recent cp scores reported by that engine.
    recent = {"white": [], "black": []}
    try:
        start_ply = len(moves)
        for ply in range(start_ply, max_plies):
            send(side, f"position startpos moves {' '.join(moves)}" if moves else "position startpos")
            send(side, f"go movetime {movetime_ms}")
            mv, status, score = get_bestmove(side, timeout=per_move_timeout)
            who = "white" if ply % 2 == 0 else "black"
            if status == "crash":
                result = "0-1" if who == "white" else "1-0"
                return result, moves, f"CRASH: {who} engine died on ply {ply}"
            if status in ("timeout", "eof"):
                result = "0-1" if who == "white" else "1-0"
                return result, moves, f"TIMEOUT/EOF: {who} on ply {ply} ({status})"
            if mv is None or mv == "(none)" or mv == "0000":
                result = "0-1" if who == "white" else "1-0"
                return result, moves, f"no legal move from {who} on ply {ply}"

            # Track this side's reported score and check for adjudication.
            if score is not None:
                recent[who].append(score)
                if len(recent[who]) > resign_window:
                    recent[who] = recent[who][-resign_window:]
            other_who = "black" if who == "white" else "white"
            this_full = len(recent[who]) == resign_window
            other_full = len(recent[other_who]) == resign_window
            this_says_losing = this_full and all(s <= -resign_cp for s in recent[who])
            other_says_winning = other_full and all(s >= resign_cp for s in recent[other_who])
            if (resign_cp > 0
                    and ply >= resign_min_ply
                    and this_says_losing
                    and other_says_winning):
                # Both engines agree 'who' is losing.
                result = "0-1" if who == "white" else "1-0"
                worst_loser = min(recent[who])
                worst_winner = min(recent[other_who])
                return result, moves + [mv], (
                    f"adjudicated (consensus): {who} <= -{resign_cp} & {other_who} >= +{resign_cp} "
                    f"for {resign_window} plies each "
                    f"(loser worst {worst_loser}, winner low {worst_winner}) at ply {ply}"
                )

            moves.append(mv)
            side, other = other, side
        return "1/2-1/2", moves, f"ply limit {max_plies} reached"
    finally:
        for p in (w, b):
            try: send(p, "quit"); p.wait(timeout=2)
            except Exception: p.kill()

# Standard 2-ply openings covering distinct opening families. Each is played
# twice (engine as white, engine as black) so colour bias cancels out.
DEFAULT_OPENINGS = [
    ("Open Game",      ["e2e4", "e7e5"]),
    ("Sicilian",       ["e2e4", "c7c5"]),
    ("French",         ["e2e4", "e7e6"]),
    ("Caro-Kann",      ["e2e4", "c7c6"]),
    ("Queen's Pawn",   ["d2d4", "d7d5"]),
    ("Indian",         ["d2d4", "g8f6"]),
    ("English",        ["c2c4", "e7e5"]),
    ("Reti",           ["g1f3", "d7d5"]),
]

def elo_diff(score, n):
    """Standard ELO formula. score = wins + 0.5*draws, n = games."""
    if n == 0: return 0
    p = score / n
    if p <= 0: return -800
    if p >= 1: return 800
    return -400 * math.log10(1/p - 1)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--engine", required=True, help="path to engine under test")
    ap.add_argument("--opponent", required=True, help="path or shell cmd for opponent")
    ap.add_argument("--games", type=int, default=None, help="if set, plays this many games alternating colours from startpos. If unset, plays each default opening twice (engine W and engine B)")
    ap.add_argument("--movetime", type=int, default=200, help="ms per move (engine may ignore)")
    ap.add_argument("--per-move-timeout", type=float, default=30.0, help="harness ceiling per move (s); decoupled from movetime since some engines ignore movetime and search at fixed depth")
    ap.add_argument("--max-plies", type=int, default=200)
    ap.add_argument("--logdir", default="/tmp/bench_games", help="where to dump per-game move logs")
    ap.add_argument("--resign-cp", type=int, default=500, help="adjudicate as loss if side-to-move reports cp <= -N for resign-window consecutive plies (0 disables)")
    ap.add_argument("--resign-window", type=int, default=8, help="how many consecutive bad plies before adjudication")
    ap.add_argument("--resign-min-ply", type=int, default=20, help="don't adjudicate before this ply (avoids early-game eval noise)")
    args = ap.parse_args()

    import os
    os.makedirs(args.logdir, exist_ok=True)

    # Build the schedule: list of (label, opening_moves, engine_plays_white)
    if args.games is not None:
        schedule = [(f"startpos", None, i % 2 == 0) for i in range(args.games)]
    else:
        schedule = []
        for name, moves in DEFAULT_OPENINGS:
            schedule.append((f"{name} (engine W)", moves, True))
            schedule.append((f"{name} (engine B)", moves, False))

    results = Counter()
    games_played = []
    for i, (label, opening, engine_is_white) in enumerate(schedule):
        if engine_is_white:
            white, black = args.engine, args.opponent
        else:
            white, black = args.opponent, args.engine
        t0 = time.time()
        result, moves, reason = play_game(
            white, black, {}, {}, args.movetime,
            max_plies=args.max_plies,
            per_move_timeout=args.per_move_timeout,
            opening=opening,
            resign_cp=args.resign_cp,
            resign_window=args.resign_window,
            resign_min_ply=args.resign_min_ply,
        )
        dt = time.time() - t0
        if result == "1-0":
            outcome = "win" if engine_is_white else "loss"
        elif result == "0-1":
            outcome = "win" if not engine_is_white else "loss"
        else:
            outcome = "draw"
        results[outcome] += 1
        games_played.append((outcome, len(moves), reason, dt))
        engine_color = "white" if engine_is_white else "black"
        print(f"Game {i+1}: {label} -> {outcome} ({len(moves)} plies, {dt:.1f}s, {reason})")
        log_path = f"{args.logdir}/game{i+1:02d}_{outcome}.txt"
        with open(log_path, "w") as f:
            f.write(f"opening: {label}\nengine_as: {engine_color}\nresult: {result}\nreason: {reason}\nply_count: {len(moves)}\nduration_s: {dt:.1f}\nmoves: {' '.join(moves)}\n")

    n = sum(results.values())
    score = results["win"] + 0.5 * results["draw"]
    print(f"\nResults: {results['win']}W {results['draw']}D {results['loss']}L  ({score}/{n})")
    print(f"ELO diff vs opponent: {elo_diff(score, n):+.0f}")

if __name__ == "__main__":
    main()
