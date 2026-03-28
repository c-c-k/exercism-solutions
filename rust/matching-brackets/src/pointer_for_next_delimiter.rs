/// Benchmarks:
/// "[]" :
///        11ns (R²=0.999, 89943765 iterations in 167 samples)
/// "(((185 + 223.85) * 15) - 543)/2" :
///       444ns (R²=0.999, 2404623 iterations in 129 samples)
/// "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)" :
///     1.191µs (R²=0.999, 842798 iterations in 118 samples)
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static DELIMITERS: LazyLock<HashMap<char, char>> =
    LazyLock::new(|| HashMap::from_iter([('(', ')'), ('[', ']'), ('{', '}')]));
static R_DELIMITERS: LazyLock<HashSet<&char>> = LazyLock::new(|| DELIMITERS.values().collect());

/// Check if a string has balanced delimiters.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut next_r_delimeter: Option<&char> = None;
    let mut r_delimeters_stack: Vec<&char> = Vec::new();

    for char in string.chars() {
        if Some(&char) == next_r_delimeter {
            next_r_delimeter = r_delimeters_stack.pop();
            continue;
        }
        if let Some(new_r_delimeter) = DELIMITERS.get(&char) {
            if let Some(next_r_delimeter) = next_r_delimeter {
                r_delimeters_stack.push(next_r_delimeter);
            }
            next_r_delimeter = Some(new_r_delimeter);
            continue;
        }
        if R_DELIMITERS.get(&char).is_some() {
            return false;
        }
    }

    r_delimeters_stack.is_empty() && next_r_delimeter.is_none()
}
