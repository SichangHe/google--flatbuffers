use flatbuffers::{Follow, InvalidFlatbuffer, Verifiable, Verifier, VerifierOptions};
use std::panic::{catch_unwind, AssertUnwindSafe};

struct Probe;

impl<'buf> Follow<'buf> for Probe {
    type Inner = usize;

    unsafe fn follow(_buf: &'buf [u8], loc: usize) -> Self::Inner {
        loc
    }
}

impl Verifiable for Probe {
    fn run_verifier(v: &mut Verifier, pos: usize) -> Result<(), InvalidFlatbuffer> {
        v.in_buffer::<u8>(pos)
    }
}

fn check_case(len: usize, encoded_offset: u32) -> Result<(), String> {
    let mut data = vec![0xa5; len];
    for (dst, src) in data.iter_mut().zip(encoded_offset.to_le_bytes()) {
        *dst = src;
    }
    let expected = len >= 4 && (encoded_offset as usize) < len;
    let actual = catch_unwind(AssertUnwindSafe(|| {
        flatbuffers::root_with_opts::<Probe>(&VerifierOptions::default(), &data)
    }))
    .map_err(|_| format!("panic len={len} offset={encoded_offset}"))?;
    match (expected, actual) {
        (true, Ok(pos)) if pos == encoded_offset as usize => Ok(()),
        (false, Err(_)) => Ok(()),
        (expected, actual) => Err(format!(
            "mismatch len={len} offset={encoded_offset} expected_success={expected} actual={actual:?}"
        )),
    }
}

fn next_u64(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn main() -> Result<(), String> {
    let mut cases = 0_u64;
    for len in 0..=64 {
        for offset in 0..=128 {
            check_case(len, offset)?;
            cases += 1;
        }
        for offset in [u32::MAX, len as u32, len.saturating_sub(1) as u32] {
            check_case(len, offset)?;
            cases += 1;
        }
    }

    let seed = 0x6a09_e667_f3bc_c909_u64;
    let mut state = seed;
    for _ in 0..100_000 {
        let len = (next_u64(&mut state) % 1025) as usize;
        let offset = next_u64(&mut state) as u32;
        check_case(len, offset)?;
        cases += 1;
    }

    println!("counterexample_search cases={cases} seed=0x{seed:016x} mismatches=0 panics=0");
    Ok(())
}
