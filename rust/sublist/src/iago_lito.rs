// NOTE:Solution by @iago-lito copied verbatim from
// <https://exercism.org/tracks/rust/exercises/sublist/solutions/iago-lito>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `huge_sublist_not_in_huge_list_2` by @bobahop
// Result: (too long)

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    use Comparison::*;
    match (first.len(), second.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (a, b) => {
            // Search the smallest within the largest.
            let revert = a > b;
            let equal_size = a == b;
            let (small, large) = if revert {
                (second, first)
            } else {
                (first, second)
            };
            // Check matches against a sliding window.
            let mut hit = false;
            for window in large.windows(small.len()) {
                // Get a speedup from avoiding preparation
                // of a slice comparison that could end up early most of the time.
                if window[0] == small[0] && window[1..] == small[1..] {
                    hit = true;
                    break;
                }
            }
            match (hit, revert, equal_size) {
                (true, false, false) => Sublist,
                (true, true, false) => Superlist,
                (true, _, true) => Equal,
                _ => Unequal,
            }
        }
    }
}
