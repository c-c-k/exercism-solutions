// NOTE: Linear brute force algorithm

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_is_super_list = _is_superlist_of(first_list, second_list);
    let second_is_super_list = first_is_super_list && first_list.len() == second_list.len()
        || _is_superlist_of(second_list, first_list);
    match (first_is_super_list, second_is_super_list) {
        (true, true) => Comparison::Equal,
        (false, true) => Comparison::Sublist,
        (true, false) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

fn _is_superlist_of(super_list: &[i32], sub_list: &[i32]) -> bool {
    _find_sublists(super_list, sub_list).0
}

fn _find_sublists(super_list: &[i32], sub_list: &[i32]) -> (bool, Vec<usize>) {
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
