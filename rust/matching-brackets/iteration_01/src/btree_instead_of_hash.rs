/// Benchmarks:
/// "[]" :
///         8ns (R²=1.000, 119715154 iterations in 170 samples)
/// "(((185 + 223.85) * 15) - 543)/2" :
///       166ns (R²=1.000, 6236992 iterations in 139 samples)
/// "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)" :
///       501ns (R²=0.999, 2186020 iterations in 128 samples)
use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

static DELIMITERS: LazyLock<BTreeMap<char, char>> =
    LazyLock::new(|| BTreeMap::from_iter([('(', ')'), ('[', ']'), ('{', '}')]));
static R_DELIMITERS: LazyLock<BTreeSet<&char>> = LazyLock::new(|| DELIMITERS.values().collect());

/// Check if a string has balanced delimiters.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut next_r_delimeter: Option<char> = None;
    let mut r_delimeters_stack: Vec<char> = Vec::new();

    for char in string.chars() {
        if Some(char) == next_r_delimeter {
            next_r_delimeter = r_delimeters_stack.pop();
            continue;
        }
        if let Some(new_r_delimeter) = DELIMITERS.get(&char) {
            if let Some(next_r_delimeter) = next_r_delimeter {
                r_delimeters_stack.push(next_r_delimeter);
            }
            next_r_delimeter = Some(*new_r_delimeter);
            continue;
        }
        if R_DELIMITERS.get(&char).is_some() {
            return false;
        }
    }

    r_delimeters_stack.is_empty() && next_r_delimeter.is_none()
}

