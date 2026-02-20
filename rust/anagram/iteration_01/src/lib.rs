use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sort_word(&word_lower);

    possible_anagrams
        .iter()
        .filter(|candidate| {
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
