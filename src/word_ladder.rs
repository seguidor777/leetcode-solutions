use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut states = HashMap::new();
        let mut visited = HashSet::new();
        let word_length = begin_word.len();

        for word in word_list {
            for i in 0..word_length {
                states
                    .entry(format!("{}*{}", &word[..i], &word[i + 1..]))
                    .or_insert(vec![])
                    .push(word.clone());
            }
        }

        let mut sequence: VecDeque<(String, i32)> =
            vec![(begin_word.clone(), 1)].into_iter().collect();

        visited.insert(begin_word);

        while !sequence.is_empty() {
            let (current_word, level) = sequence.pop_front().unwrap();

            for i in 0..word_length {
                let state = format!("{}*{}", &current_word[..i], &current_word[i + 1..]);

                if let Some(words_matching) = states.get(&state) {
                    for next_word in words_matching {
                        if next_word == &end_word {
                            return level + 1;
                        }

                        if !visited.contains(next_word) {
                            visited.insert(next_word.to_string());
                            sequence.push_back((next_word.to_string(), level + 1));
                        }
                    }

                    states.remove(&state).unwrap();
                }
            }
        }
        0
    }
}
