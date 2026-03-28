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

    let mut double = false;
    let mut sum = 0;

    for char in code.chars().rev() {
        if char.is_whitespace() {
            continue;
        }

        if !char.is_ascii_digit() {
            return false;
        };
        let mut digit = char.to_digit(RADIX).unwrap();
        if double {
            match digit {
                5..9 => digit = digit * 2 - 9,
                1..5 => digit *= 2,
                _ => (),
            }
        }
        sum += digit;
        double = !double;
    }

    sum % 10 == 0
}
