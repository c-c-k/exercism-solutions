/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // clean
    let mut digits = Vec::with_capacity(code.len());
    let digit_chars = code.chars().filter(|char| *char != ' ');
    for char in digit_chars {
        digits.push(if let Some(digit) = char.to_digit(10) {
            digit
        } else {
            return false;
        });
    }
    // insure minimal length
    if digits.len() <= 1 {
        return false;
    };
    // check checksum
    let mut double = digits.len() % 2 != 0;
    let sum = digits.iter().fold(0, |sum, digit| {
        double = !double;
        let d = if double && *digit != 9 {
            *digit * 2 % 9
        } else {
            *digit
        };
        sum + d
    });
    sum % 10 == 0
}
