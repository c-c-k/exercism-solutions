// NOTE:
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `nth(10_000)`
// Result: 159.842201ms (R²=0.979, 6 iterations in 5 samples)

use std::collections::HashSet;
// use std::sync::LazyLock;

const SEARCH_LIMIT: u32 = 1_000_005;
// static PRIMES: LazyLock<Vec<u32>> = LazyLock::new(_sieve_of_eratosthenes);

fn _sieve_of_eratosthenes() -> Vec<u32> {
    let search_limit_sqrt = SEARCH_LIMIT.isqrt();
    let mut primes: Vec<u32> = Vec::with_capacity(search_limit_sqrt as usize);
    let mut non_primes: HashSet<u32> = HashSet::with_capacity(SEARCH_LIMIT as usize);
    for num in 2..=search_limit_sqrt {
        if !non_primes.contains(&num) {
            primes.push(num);
            for non_prime in (num.pow(2)..=SEARCH_LIMIT).step_by(num as usize) {
                non_primes.insert(non_prime);
            }
        }
    }
    for num in search_limit_sqrt..=SEARCH_LIMIT {
        if !non_primes.contains(&num) {
            primes.push(num);
        }
    }
    primes
}

pub fn nth(n: u32) -> u32 {
    let primes = _sieve_of_eratosthenes();
    *primes.get(n as usize).unwrap()
}
