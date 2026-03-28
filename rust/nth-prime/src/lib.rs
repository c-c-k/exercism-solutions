// NOTE:
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `nth(10_000)`
// Result: 1ns (R²=1.000, 500079936 iterations in 185 samples)

use std::collections::HashSet;
use std::sync::LazyLock;

pub mod bobahop;
pub mod hlissner;
pub mod lewisclement;
pub mod sieve_step_by;
pub mod sieve_while;
pub mod stevensonmt;

const SEARCH_LIMIT: u32 = 1_000_005;
static PRIMES: LazyLock<Vec<u32>> = LazyLock::new(_sieve_of_eratosthenes);

fn _sieve_of_eratosthenes() -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::with_capacity(SEARCH_LIMIT.isqrt() as usize);
    let mut non_primes: HashSet<u32> = HashSet::with_capacity(SEARCH_LIMIT as usize);
    for num in 2..=SEARCH_LIMIT.isqrt() {
        if !non_primes.contains(&num) {
            primes.push(num);
            for non_prime in (num.pow(2)..=SEARCH_LIMIT).step_by(num as usize) {
                non_primes.insert(non_prime);
            }
        }
    }
    for num in SEARCH_LIMIT.isqrt()..=SEARCH_LIMIT {
        if !non_primes.contains(&num) {
            primes.push(num);
        }
    }
    primes
}

pub fn nth(n: u32) -> u32 {
    *PRIMES.get(n as usize).unwrap()
}
