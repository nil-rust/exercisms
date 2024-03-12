use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    let word_map = convert_to_str_map(&word.to_lowercase());

    for possible_anagram in possible_anagrams {
        let possible_word = *possible_anagram;
        if possible_word.to_lowercase() == word.to_lowercase() {
            continue
        }

        let possible_word_map = convert_to_str_map(&possible_word.to_lowercase());

        if possible_word_map == word_map {
            set.insert(*possible_anagram);
        }
    }

    set
}

fn convert_to_str_map(word: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for letter in word.chars() {
        let count = map.get(&letter).unwrap_or(&0);

        map.insert(letter, count + 1);
    }

    map
}
