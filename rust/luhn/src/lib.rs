// Mixed from:
// bobahop's solution
// <https://exercism.org/tracks/rust/exercises/luhn/solutions/bobahop>
// lewisclement's solution
// <https://exercism.org/tracks/rust/exercises/luhn/solutions/lewisclement>
// LudwigStecher's solution
// <https://exercism.org/tracks/rust/exercises/luhn/solutions/LudwigStecher>

const RADIX: u32 = 10;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Reject empty or 1 digit codes
    let code = code.trim();
    if code.len() < 2 {
        return false;
    }
    let mut keep = false;

    code.chars()
        .filter(|char| !char.is_whitespace())
        .try_rfold(0, |sum, char| {
            keep = !keep;
            char.to_digit(RADIX)
                .map(|digit| {
                    if keep {
                        digit
                    } else {
                        match digit {
                            5..9 => digit * 2 - 9,
                            1..5 => digit * 2,
                            _ => digit,
                        }
                    }
                })
                .map(|digit| sum + digit)
        })
        .is_some_and(|sum| sum % 10 == 0)
}
