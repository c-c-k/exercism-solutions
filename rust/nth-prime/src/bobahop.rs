// NOTE:Solution by @bobahop copied verbatim from
// <https://exercism.org/tracks/rust/exercises/nth-prime/solutions/bobahop>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `nth(10_000)`
// Result:  2.036468ms (R²=0.999, 535 iterations in 41 samples)

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes: Vec<u32> = vec![];
    let mut found_count = 0;
    let mut step_child = 3;
    loop {
        let red_headed_step_child = (step_child as f64).sqrt() as u32;
        let mut may_be_prime = true;
        for prime in &primes {
            if step_child % prime == 0 {
                may_be_prime = false;
                break;
            }
            if *prime > red_headed_step_child {
                break;
            }
        }
        if may_be_prime {
            primes.push(step_child);
            found_count += 1;
            if found_count == n {
                return step_child;
            }
        }
        step_child += 2;
    }
}
