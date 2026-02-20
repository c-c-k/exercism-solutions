use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // TODO(maybe someday): Actual benchmarking and optimization
    if limit / 10 > factors.len() {
        sum_from_factors(limit, factors)
    } else {
        sum_from_range(limit, factors)
    }
}

pub fn sum_from_factors(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|&factor| (factor..limit).step_by(factor as usize))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

pub fn sum_from_range(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| {
            factors
                .iter()
                .any(|&factor| factor != 0 && num % factor == 0)
        })
        .sum()
}
