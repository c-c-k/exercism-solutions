// NOTE: This is a solution optimized for multiple queries while also
//       being able to handle queries for big numbers and big primes.
// Adjusted from:
// 1. <https://www.geeksforgeeks.org/prime-factor/>
// 2. <https://www.geeksforgeeks.org/prime-factorization-using-sieve-olog-n-multiple-queries/>
// 3. <https://www.geeksforgeeks.org/least-prime-factor-of-numbers-till-n/>
// 4. <https://exercism.org/tracks/lua/exercises/prime-factors/solutions/carlospavanetti>

use std::sync::LazyLock;

const SEARCH_LIMIT: usize = 10_000_000;
static CACHE: LazyLock<Cache> = LazyLock::new(_generate_cache);

struct Cache {
    lpf: Vec<usize>,
    primes: Vec<usize>,
}

fn _generate_cache() -> Cache {
    let mut primes = Vec::with_capacity(SEARCH_LIMIT);
    let mut lpf = vec![2; SEARCH_LIMIT];
    lpf[0] = 0;
    lpf[1] = 1;

    for num in (3..SEARCH_LIMIT).step_by(2) {
        if lpf[num] == 2 {
            primes.push(num);
            for multiple in (num..SEARCH_LIMIT).step_by(2 * num) {
                if lpf[multiple] == 2 {
                    lpf[multiple] = num
                }
            }
        };
    }
    primes.shrink_to_fit();

    Cache { lpf, primes }
}

fn extract_pf_from_big_numbers(n: &mut usize, prime_factors: &mut Vec<u64>) {
    for &prime in &CACHE.primes {
        if *n <= SEARCH_LIMIT {
            break;
        }

        while n.is_multiple_of(prime) {
            prime_factors.push(prime as u64);
            *n /= prime
        }
    }
}

fn extract_factor(factor: usize, n: &mut usize, prime_factors: &mut Vec<u64>) {
    while n.is_multiple_of(factor) {
        prime_factors.push(factor as u64);
        *n /= factor;
    }
}

fn extract_big_prime_factors(n: &mut usize, prime_factors: &mut Vec<u64>) {
    let max_seed = n.isqrt() / 6 + 1;
    let min_seed = SEARCH_LIMIT / 6;

    for seed in min_seed..=max_seed {
        if *n == 1 {
            break;
        }
        extract_factor(seed * 6 - 1, n, prime_factors);
        extract_factor(seed * 6 + 1, n, prime_factors);
    }

    if *n != 1 {
        prime_factors.push(*n as u64);
    }
}

fn extract_normal_prime_factors(n: &mut usize, prime_factors: &mut Vec<u64>) {
    while *n > 1 {
        let prime_factor = CACHE.lpf[*n];
        while n.is_multiple_of(prime_factor) {
            prime_factors.push(prime_factor as u64);
            *n /= prime_factor;
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n as usize;
    let mut prime_factors = Vec::new();

    if n > SEARCH_LIMIT {
        extract_pf_from_big_numbers(&mut n, &mut prime_factors);
    };
    if n > SEARCH_LIMIT {
        extract_big_prime_factors(&mut n, &mut prime_factors);
    } else {
        extract_normal_prime_factors(&mut n, &mut prime_factors);
    };

    prime_factors
}
