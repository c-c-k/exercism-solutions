// NOTE:Boyer-Moore Algorithm
// <https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm>
// <https://www.geeksforgeeks.org/boyer-moore-algorithm-for-pattern-searching/>
// <https://www.geeksforgeeks.org/boyer-moore-algorithm-good-suffix-heuristic/>
// **The following site seems to be AI generated content**
// <https://www.codingdrills.com/tutorial/introduction-to-searching-algorithms/sublist-search-python>
// --
// Benchmarked:
// With <https://github.com/asayers/easybench-rs>
// on: `huge_sublist_not_in_huge_list_2` by @bobahop
// Result: 29.362986ms (R²=0.997, 34 iterations in 15 samples)

use std::{collections::HashMap, fmt::Debug, hash::Hash};

fn _build_bad_char_table<T>(sub_list: &[T]) -> HashMap<&T, usize>
where
    T: PartialEq + Eq + Hash,
{
    HashMap::from_iter(sub_list.iter().enumerate().map(|(i, v)| (v, i)))
}

fn _preprocess_general_case<T>(
    good_suffix_table: &mut [usize],
    borders: &mut [usize],
    sub_list: &[T],
) where
    T: PartialEq + Eq,
{
    let mut posiotion = sub_list.len();
    let mut border = borders.len();
    borders[posiotion] = border;

    while posiotion > 0 {
        while border < borders.len() && sub_list[posiotion - 1] != sub_list[border - 1] {
            if good_suffix_table[border] == 0 {
                good_suffix_table[border] = border - posiotion;
            };

            border = borders[border]
        }

        posiotion -= 1;
        border -= 1;
        borders[posiotion] = border;
    }
}

fn _preprocess_fallback_case(good_suffix_table: &mut [usize], borders: &mut [usize]) {
    let mut border = borders[0];

    for (position, gst_entry) in good_suffix_table.iter_mut().enumerate().take(borders.len()) {
        if *gst_entry == 0 {
            *gst_entry = border;
        };

        if position == border {
            border = borders[border]
        };
    }
}

fn _build_good_suffix_table<T>(sub_list: &[T]) -> Vec<usize>
where
    T: PartialEq + Eq,
{
    let mut borders: Vec<usize> = vec![0; sub_list.len() + 1];
    let mut good_suffix_table: Vec<usize> = vec![0; sub_list.len() + 1];

    _preprocess_general_case(&mut good_suffix_table, &mut borders, sub_list);
    _preprocess_fallback_case(&mut good_suffix_table, &mut borders);

    good_suffix_table
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

    let bad_char_table = _build_bad_char_table(sub_list);
    let good_suffix_table = _build_good_suffix_table(sub_list);

    let mut match_end = sub_list.len() - 1;
    let mut bad_char_skip: usize;
    let mut good_suffix_skip: usize;

    while match_end < super_list.len() {
        let mut sub_list_index = sub_list.len() - 1;
        let mut super_list_index = match_end;
        let found_match = loop {
            if super_list[super_list_index] != sub_list[sub_list_index] {
                break false;
            }
            if sub_list_index == 0 {
                break true;
            }
            super_list_index -= 1;
            sub_list_index -= 1;
        };

        if found_match {
            match_positions.push(super_list_index);

            bad_char_skip = sub_list.len()
                - bad_char_table
                    .get(&super_list[match_end])
                    .map_or(sub_list.len(), |v| *v);
            good_suffix_skip = good_suffix_table[0];
        } else {
            bad_char_skip = sub_list_index.saturating_sub(
                bad_char_table
                    .get(&super_list[super_list_index])
                    .map_or(sub_list.len(), |v| *v),
            );
            good_suffix_skip = good_suffix_table[sub_list_index + 1];
        }

        match_end += bad_char_skip.max(good_suffix_skip);
    }

    (!match_positions.is_empty(), match_positions)
}
