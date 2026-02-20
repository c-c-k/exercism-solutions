use easybench::bench;

fn main() {
    // println!("Result: {}", bench(|| nth_prime::nth(10_000)));
    // println!("Result: {}", bench(|| nth_prime::bobahop::nth(10_000)));
    // println!("Result: {}", bench(|| nth_prime::lewisclement::nth(10_000)));
    // println!("Result: {}", bench(|| nth_prime::hlissner::nth(10_000)));
    // println!("Result: {}", bench(|| nth_prime::stevensonmt::nth(10_000)));
    // println!(
    //     "Result: {}",
    //     bench(|| nth_prime::sieve_step_by::nth(10_000))
    // );
    println!("Result: {}", bench(|| nth_prime::sieve_while::nth(10_000)));
}
