use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sort_word(&word_lower);

    possible_anagrams
        .iter()
        // `&str` for `&'a str` in `&[&'a str]`
        // `&&str` for `iter` over `&[&str]`
        // `&&&str` for `filter` over `iter<[&&str]>`
        // `&&candidate` to get a type of `&str` for `candidate` from
        //   the passed in `&&&str` type.
        // `&&&candidate` can't be used to get a type of `str`
        //   for `candidate` from the passed in `&&&str` type
        //   because local variables must have a known size at
        //   compile time which `str` doesn't.
        // .filter(|&&candidate: &&&str| {
        // `&&candidate` isn't actually necessary and `candidate` can
        //   be used instead due to implicit deref coercion in the
        //   `to_lowercase()` method using it later on.
        .filter(|candidate| {
            // `*candidate` to turn the type `&str` `candidate` parameter
            //   into the `str` type required by `to_lowercase()`.
            // let candidate_lower = (*candidate).to_lowercase();
            // `*candidate` isn't actually necessary and `candidate` can
            //   be used instead due to implicit deref coercion.
            let candidate_lower = candidate.to_lowercase();
            let candidate_sorted = sort_word(&candidate_lower);
            candidate_lower != word_lower && candidate_sorted == word_sorted
        })
        .copied()
        .collect()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut word: Vec<char> = word.chars().collect();
    word.sort_unstable();
    word
}
