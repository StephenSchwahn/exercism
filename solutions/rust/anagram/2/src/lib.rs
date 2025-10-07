use std::collections::HashSet;

fn sort_word<'a>(word: &'a str) -> String {
    let mut char_vec: Vec<char> = word.to_lowercase().chars().collect();
    char_vec.sort_unstable();

    char_vec.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = sort_word(word);
    let lowercased = word.to_lowercase();

    HashSet::from_iter(
        possible_anagrams
            .to_owned()
            .into_iter()
            .filter(|anagram_word| {
                let compare_str: String = sort_word(&anagram_word);
            
                compare_str == sorted_word && anagram_word.to_lowercase() != lowercased
            })
    )
}
