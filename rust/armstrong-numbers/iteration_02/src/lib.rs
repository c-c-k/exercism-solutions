pub fn is_armstrong_number(num: u32) -> bool {
    let Some(power) = num.checked_ilog10().map(|p| p + 1) else {
        return true;
    };

    let mut left = num;
    let mut sum = 0;

    num == loop {
        sum += (left % 10).pow(power);
        left /= 10;
        if left == 0 {
            break sum;
        }
    }
}
