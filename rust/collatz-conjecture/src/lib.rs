/// Benchmarks:
// 1000000 :
//        99ns (R²=0.999, 10044744 iterations in 144 samples)
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut n = n;
    loop {
        n = match n {
            1 => break Some(steps),
            even if even % 2 == 0 => {
                steps += 1;
                even / 2
            }
            _ => {
                steps += 2;
                n.checked_mul(3)?.div_ceil(2)
            }
        };
    }
}
