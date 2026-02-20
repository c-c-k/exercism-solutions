// use boyer_moore::find_sublists;
use kmp::find_sublists;
// use linear::find_sublists;
use std::{fmt::Debug, hash::Hash};

pub mod adeveloper385;
pub mod bobahop;
pub mod boyer_moore;
pub mod iago_lito;
pub mod kmp;
pub mod linear;
pub mod vagrawal;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(first_list: &[T], second_list: &[T]) -> Comparison
where
    T: PartialEq + Eq + Hash + Debug,
{
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

pub fn _is_superlist_of<T>(super_list: &[T], sub_list: &[T]) -> bool
where
    T: PartialEq + Eq + Hash + Debug,
{
    find_sublists(super_list, sub_list).0
}
