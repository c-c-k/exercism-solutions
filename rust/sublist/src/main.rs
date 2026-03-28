use easybench::bench;
// use sublist::*;
// use sublist::adeveloper385::*;
// use sublist::bobahop::*;
// use sublist::iago_lito::*;
use sublist::vagrawal::*;
fn main() {
    println!("Result: {}", bench(huge_sublist_not_in_huge_list_2))
}
fn huge_sublist_not_in_huge_list_2() {
    let v1: Vec<u64> = vec![0; 1_000_000];
    let mut v2: Vec<u64> = vec![0; 500_000];
    v2.push(1);
    assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}
