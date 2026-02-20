// NOTE:Solution by @hlissner copied verbatim from
// <https://exercism.org/tracks/rust/exercises/nth-prime/solutions/hlissner>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `nth(10_000)`
// Result: 3.67657ms (R²=0.999, 269 iterations in 34 samples)

fn is_prime(n: u32) -> bool {
    !(2..3)
        .chain((3..(n as f32).sqrt() as u32 + 1).step_by(2))
        .any(|x| n % x == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..3)
        .chain((3..).filter(|x| is_prime(*x)))
        .nth(n as usize)
        .unwrap()
}
