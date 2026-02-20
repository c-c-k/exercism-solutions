// Copied from JaneL's solution
// <https://exercism.org/tracks/rust/exercises/luhn/solutions/JaneL>

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|char| !char.is_whitespace())
        .try_fold((0, 0), |(rev_index, sum), char| {
            char.to_digit(10)
                .map(|digit| {
                    if rev_index % 2 == 1 && digit != 9 {
                        digit * 2 % 9
                    } else {
                        digit
                    }
                })
                .map(|digit| (rev_index + 1, sum + digit))
        })
        .is_some_and(|(count, sum)| sum % 10 == 0 && count > 1)
}
