pub fn w_single_push_targets(wpawns: u64, empty: u64) -> u64 {
    (wpawns << 8) & empty
}

pub fn w_double_push_targets(wpawns: u64, empty: u64) -> u64 {
    const RANK4: u64 = 0x00000000FF000000;
    let single_push_targets = w_single_push_targets(wpawns, empty);
    (single_push_targets << 8) & empty & RANK4
}

pub fn b_single_push_targets(bpawns: u64, empty: u64) -> u64 {
    (bpawns >> 8) & empty
}

pub fn b_double_push_targets(bpawns: u64, empty: u64) -> u64 {
    const RANK5: u64 = 0x000000FF00000000;
    let single_push_targets = b_single_push_targets(bpawns, empty);
    (single_push_targets >> 8) & empty & RANK5
}

pub fn w_pawns_able_to_push(wpawns: u64, empty: u64) -> u64 {
    (empty >> 8) & wpawns
}

pub fn w_pawns_able_to_double_push(wpawns: u64, empty: u64) -> u64 {
    const RANK4: u64 = 0x00000000FF000000;
    let empty_rank3 = (empty & RANK4) >> 8 & empty;
    w_pawns_able_to_push(wpawns, empty_rank3)
}

pub fn b_pawns_able_to_push(bpawns: u64, empty: u64) -> u64 {
    (empty << 8) & bpawns
}

pub fn b_pawns_able_to_double_push(bpawns: u64, empty: u64) -> u64 {
    const RANK5: u64 = 0x000000FF00000000;
    let empty_rank6 = (empty & RANK5) << 8 & empty;
    b_pawns_able_to_push(bpawns, empty_rank6)
}
