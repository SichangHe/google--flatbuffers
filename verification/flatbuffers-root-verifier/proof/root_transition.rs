use vstd::prelude::*;

verus! {

pub open spec fn sat_add_u64(left: nat, right: nat) -> nat {
    if left + right > 18_446_744_073_709_551_615 {
        18_446_744_073_709_551_615
    } else {
        left + right
    }
}

pub open spec fn production_probe_accepts(len: nat, offset: nat) -> bool {
    sat_add_u64(0, 4) <= len
        && sat_add_u64(offset, 0) == offset
        && sat_add_u64(offset, 1) <= len
}

pub open spec fn oracle_accepts(len: nat, offset: nat) -> bool {
    4 <= len && offset < len
}

proof fn root_offset_addition_cannot_saturate(offset: u32)
    ensures
        sat_add_u64(offset as nat, 0) == offset as nat,
        sat_add_u64(offset as nat, 1) == offset as nat + 1,
{
}

proof fn production_range_matches_oracle(len: u64, offset: u32)
    ensures
        production_probe_accepts(len as nat, offset as nat)
            == oracle_accepts(len as nat, offset as nat),
{
}

fn checked_probe_position(len: u64, offset: u32) -> (result: Option<u64>)
    ensures
        match result {
            Some(pos) => pos == (offset as u64) && 4 <= len && pos < len,
            None => len < 4 || len <= (offset as u64),
        },
{
    let pos = offset as u64;
    if len < 4 || len <= pos {
        None
    } else {
        Some(pos)
    }
}

proof fn accepted_position_is_safe_for_unchecked_follow(len: u64, offset: u32)
    requires
        oracle_accepts(len as nat, offset as nat),
    ensures
        4 <= len,
        (offset as u64) < len,
        (offset as u64) <= 4_294_967_295u64,
{
}

fn main() {
    let accepted = checked_probe_position(5u64, 4u32);
    assert(accepted == Some(4u64));
    let rejected = checked_probe_position(4u64, 4u32);
    assert(rejected == None::<u64>);
}

}
