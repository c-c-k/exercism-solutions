pub fn is_armstrong_number(num: u32) -> bool {
    let mut left = num;
    let mut digits: Vec<u32> = Vec::new();
    while left != 0 {
        digits.push(left % 10);
        left /= 10;
    }

    let num_len = digits.len() as u32;
    num == digits
        .iter()
        .fold(0, |sum, digit| sum + (digit.pow(num_len)))
}
