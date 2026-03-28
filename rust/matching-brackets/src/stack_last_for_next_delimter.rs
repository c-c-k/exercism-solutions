/// Benchmarks:
/// "[]" :
///        31ns (R²=1.000, 31524734 iterations in 156 samples)
/// "(((185 + 223.85) * 15) - 543)/2" :
///       460ns (R²=1.000, 2186020 iterations in 128 samples)
/// "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)" :
///     1.231µs (R²=1.000, 842798 iterations in 118 samples)
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static DELIMITERS: LazyLock<HashMap<char, char>> =
    LazyLock::new(|| HashMap::from_iter([('(', ')'), ('[', ']'), ('{', '}')]));
static R_DELIMITERS: LazyLock<HashSet<char>> =
    LazyLock::new(|| DELIMITERS.values().cloned().collect());

/// Check if a string has balanced delimiters.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut r_delimeters_stack: Vec<char> = Vec::new();

    for char in string.chars() {
        if Some(&char) == r_delimeters_stack.last() {
            r_delimeters_stack.pop();
            continue;
        }
        if let Some(&new_r_delimeter) = DELIMITERS.get(&char) {
            r_delimeters_stack.push(new_r_delimeter);
            continue;
        }
        if R_DELIMITERS.get(&char).is_some() {
            return false;
        }
    }

    r_delimeters_stack.is_empty()
}
