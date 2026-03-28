// NOTE: Linear brute force algorithm
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `huge_sublist_not_in_huge_list_2` by @bobahop
// Result: (too long)

use std::{fmt::Debug, hash::Hash};

pub fn find_sublists<T>(super_list: &[T], sub_list: &[T]) -> (bool, Vec<usize>)
where
    T: PartialEq + Eq + Hash + Debug,
{
    if sub_list.is_empty() {
        return (true, vec![0]);
    }
    if sub_list.len() > super_list.len() {
        return (false, vec![]);
    }

    let mut match_positions: Vec<usize> = Vec::new();

    'outer: for i in 0..=(super_list.len() - sub_list.len()) {
        for j in 0..sub_list.len() {
            if sub_list[j] != super_list[i + j] {
                continue 'outer;
            }
        }
        match_positions.push(i);
    }

    (!match_positions.is_empty(), match_positions)
}
