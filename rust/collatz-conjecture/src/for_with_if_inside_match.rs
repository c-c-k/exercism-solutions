/// Benchmarks:
// 1000000 :
//       106ns (R²=0.999, 10044744 iterations in 144 samples)
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;
    for steps in 0.. {
        n = match n {
            1 => return Some(steps),
            even if even % 2 == 0 => even / 2,
            _ => n.checked_mul(3)?.checked_add(1)?,
        };
    }

    None
}
