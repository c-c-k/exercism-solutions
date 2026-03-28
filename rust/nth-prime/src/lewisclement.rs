// NOTE:Solution by @lewisclement copied verbatim from
// <https://exercism.org/tracks/rust/exercises/nth-prime/solutions/lewisclement>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `nth(10_000)`
// Result: 2.380964ms (R²=0.999, 440 iterations in 39 samples)

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut div: u32 = 5;
    while div * div <= n {
        if n % div == 0 || n % (div + 2) == 0 {
            return false;
        }
        div = div + 6;
    }

    true
}

pub fn nth(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n + 2;
    }

    (1..n).fold(1, |last, _| {
        (last + 2..).step_by(2).find(|v| is_prime(*v)).unwrap()
    })
}
