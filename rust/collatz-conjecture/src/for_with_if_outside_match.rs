/// Benchmarks:
/// 1000000 :
///       107ns (R²=1.000, 10044744 iterations in 144 samples)
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;
    for steps in 0.. {
        if n == 1 {
            return Some(steps);
        }
        n = match n % 2 {
            0 => n / 2,
            _ => n.checked_mul(3)?.checked_add(1)?,
        }
    }

    None
}
