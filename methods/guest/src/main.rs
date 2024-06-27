#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use hyle_contract::{HyleInput, HyleOutput};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: HyleInput<u32> = env::read();

    let initial_state = u32::from_be_bytes(input.initial_state.clone().try_into().unwrap());
    env::commit(&HyleOutput {
        version: 1,
        block_number: input.block_number,
        block_time: input.block_time,
        origin: input.origin,
        caller: input.caller,
        tx_hash: input.tx_hash,
        program_outputs: "Any output heehee",
        initial_state: input.initial_state,
        next_state: u32::to_be_bytes( is_prime(initial_state)).to_vec()
    })
}

fn is_prime(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }
    if n == 2 || n == 3 {
        return 0;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return 0;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return 0;
        }
        i += 6;
    }
    1
}
