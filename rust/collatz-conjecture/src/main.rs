use easybench::bench;

use collatz_conjecture::collatz;

fn main() {
    let samples = vec![1_000_000];
    for sample in samples {
        let f = || collatz(sample);
        println!("{sample:?} :\n{}", bench(f))
    }
}
