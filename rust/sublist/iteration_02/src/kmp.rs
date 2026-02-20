// NOTE:KMP algorithm
// <https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm>
// <https://www.geeksforgeeks.org/kmp-algorithm-for-pattern-searching/>
// **The following site seems to be AI generated content**
// <https://www.codingdrills.com/tutorial/introduction-to-searching-algorithms/sublist-search-python>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// On: `huge_sublist_not_in_huge_list_2` by @bobahop
// Result:  8.749104ms (R²=0.999, 120 iterations in 26 samples)

use std::vec;
use std::{fmt::Debug, hash::Hash};

fn _build_partial_match_table<T>(sub_list: &[T]) -> Vec<Option<usize>>
where
    T: PartialEq + Eq + Hash + Debug,
{
    let mut partial_match_table: Vec<Option<usize>> = vec![None; sub_list.len() + 1];
    let mut candidate = 0;

    for position in 1..sub_list.len() {
        if sub_list[position] == sub_list[candidate] {
            partial_match_table[position] = partial_match_table[candidate]
        } else {
            partial_match_table[position] = Some(candidate);
            while partial_match_table[candidate].is_some()
                && sub_list[position] != sub_list[candidate]
            {
                candidate = partial_match_table[candidate].unwrap_or(0);
            }
        }
        candidate += 1;
    }

    partial_match_table[sub_list.len()] = Some(candidate);
    partial_match_table
}

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
    let partial_match_table = _build_partial_match_table(sub_list);
    let mut position = 0;
    let mut match_start = 0;

    while position < super_list.len() {
        if sub_list[match_start] == super_list[position] {
            position += 1;
            match_start += 1;
            if match_start >= sub_list.len() {
                match_positions.push(position - match_start);
                match_start = partial_match_table[match_start].unwrap_or(0);
            }
        } else if let Some(backtrack) = partial_match_table[match_start] {
            match_start = backtrack
        } else {
            position += 1;
            match_start = 0;
        }
    }

    (!match_positions.is_empty(), match_positions)
}
