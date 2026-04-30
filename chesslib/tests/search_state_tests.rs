use chesslib::search::SearchState;
use chesslib::types::{Move, Square};

fn mv(src: Square, target: Square) -> Move {
    Move {
        src,
        target,
        promotion: None,
    }
}

#[test]
fn new_state_has_no_killers() {
    let ss = SearchState::new();
    assert_eq!(ss.killers[0][0], None);
    assert_eq!(ss.killers[0][1], None);
    assert_eq!(ss.killers[5][0], None);
}

#[test]
fn record_cutoff_quiet_move_stores_as_killer_0() {
    let mut ss = SearchState::new();
    let m = mv(Square::E2, Square::E4);
    ss.record_cutoff(3, m, 5, false);
    assert_eq!(ss.killers[3][0], Some(m));
    assert_eq!(ss.killers[3][1], None);
}

#[test]
fn record_cutoff_capture_does_not_store_killer() {
    // Captures already get priority through MVV-LVA; storing them as killers
    // would be redundant and could push real quiet killers out of the slots.
    let mut ss = SearchState::new();
    let m = mv(Square::E4, Square::D5);
    ss.record_cutoff(3, m, 5, true);
    assert_eq!(ss.killers[3][0], None);
}

#[test]
fn second_distinct_cutoff_shifts_old_killer_to_slot_1() {
    let mut ss = SearchState::new();
    let m1 = mv(Square::E2, Square::E4);
    let m2 = mv(Square::G1, Square::F3);
    ss.record_cutoff(3, m1, 5, false);
    ss.record_cutoff(3, m2, 5, false);
    assert_eq!(ss.killers[3][0], Some(m2));
    assert_eq!(ss.killers[3][1], Some(m1));
}

#[test]
fn duplicate_cutoff_does_not_shift_killers() {
    // If the same move keeps causing cutoffs, don't waste both slots on it.
    let mut ss = SearchState::new();
    let m1 = mv(Square::E2, Square::E4);
    ss.record_cutoff(3, m1, 5, false);
    ss.record_cutoff(3, m1, 5, false);
    assert_eq!(ss.killers[3][0], Some(m1));
    assert_eq!(ss.killers[3][1], None);
}

#[test]
fn record_cutoff_accumulates_history_score_by_depth_squared() {
    // History scores quiet moves by depth^2, so cutoffs deep in the tree
    // (which prune more) carry more weight.
    let mut ss = SearchState::new();
    let m = mv(Square::E2, Square::E4);
    let from = Square::E2 as usize;
    let to = Square::E4 as usize;
    ss.record_cutoff(0, m, 5, false);
    assert_eq!(ss.history[from][to], 25);
    ss.record_cutoff(0, m, 3, false);
    assert_eq!(ss.history[from][to], 25 + 9);
}

#[test]
fn capture_cutoff_does_not_increment_history() {
    let mut ss = SearchState::new();
    let m = mv(Square::E4, Square::D5);
    let from = Square::E4 as usize;
    let to = Square::D5 as usize;
    ss.record_cutoff(0, m, 5, true);
    assert_eq!(ss.history[from][to], 0);
}

#[test]
fn ply_out_of_range_is_a_no_op_not_a_panic() {
    // Defensive: search depth might exceed our killer table by accident.
    let mut ss = SearchState::new();
    let m = mv(Square::E2, Square::E4);
    ss.record_cutoff(1000, m, 5, false); // way past MAX_SEARCH_PLY
                                         // History is indexed by squares (always 0-63), so it should still update.
    let from = Square::E2 as usize;
    let to = Square::E4 as usize;
    assert_eq!(ss.history[from][to], 25);
}
