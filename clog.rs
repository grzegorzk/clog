#![allow(dead_code)]

use std::io::{self, BufRead};
use std::collections::HashMap;

struct LogFilters {
    // Each vector line stores a vector of individual words variations
    // line_filters (Vec) - collection of all log lines
    //    |
    //    |- words (Vec) - collection of word variations within log line
    //          |
    //          |- word_variations (Vec) - collection of words within word variation
    //                   |
    //                   |- word (String)
    line_filters: Vec<Vec<Vec<String>>>,
    // Each unique word from `line_filters` gets its own key
    // Each key stores references to lines containing the key
    words_hash: HashMap<String, Vec<u32>>,
    // Minimum required consequent matches to consider lines similar
    min_req_consequent_matches: u32
}

impl LogFilters {
    fn new() -> Self {
        let line_filters = Vec::new();
        let words_hash = HashMap::new();

        LogFilters {
            line_filters: line_filters,
            words_hash: words_hash,
            min_req_consequent_matches: 3
        }
    }

    fn _update_hash(&mut self, word: &String, filter_index: u32) {
        self.words_hash.entry(word.clone()).or_insert(vec![filter_index]);
        let vector_indexes = self.words_hash.get_mut(word).unwrap();
        if ! vector_indexes.contains(&filter_index) {
            vector_indexes.push(filter_index);
            vector_indexes.sort();
        }
    }

    fn _is_word_in_line_filter(&self, word: &String, filter_index: u32) -> bool {
        let line_filter = self.line_filters.get(filter_index as usize);
        if line_filter.is_none() {
            return false;
        }
        
        let line_filter = line_filter.unwrap();
        for word_alternatives in line_filter {
            for word_alternative in word_alternatives {
                if word_alternative == word {
                    return true;
                }
            }
        }
        return false;
    }

    fn _count_consequent_matches_in_line_filter(&self, words: &Vec<String>, filter_index: u32) -> u32 {
        let mut consequent_matches = 0;
        let mut max_consequent_matches = 0;
        for word in words {
            if self._is_word_in_line_filter(word, filter_index) {
                consequent_matches += 1;
            }
            else {
                if consequent_matches > max_consequent_matches {
                    max_consequent_matches = consequent_matches;
                }
                consequent_matches = 0;
            }
        }
        return max_consequent_matches;
    }

    fn _get_sorted_line_filter_indexes_with_words(&self, words: &Vec<String>) -> Vec<u32> {
        let mut line_filters_with_words: Vec<u32> = Vec::new();
        for word in words {
            if self.words_hash.get(word).is_some() {
                let vector_indexes = self.words_hash.get(word).unwrap();
                line_filters_with_words.extend(vector_indexes);
            }
        }
        line_filters_with_words.sort();
        return line_filters_with_words;
    }

    fn _get_line_filter_indexes_with_min_req_matches(&self, words: &Vec<String>) -> Vec<u32> {
        let mut line_filter_indexes_with_min_req_matches: Vec<u32> = Vec::new();
        let line_filters_with_words = self._get_sorted_line_filter_indexes_with_words(words);
        let mut matches = 0;
        let mut prev_index = -1;
        let mut last_inserted_index = -1;
        for line_filter_index in line_filters_with_words {
            if last_inserted_index == line_filter_index as i32 {
                continue;
            }
            if prev_index != line_filter_index as i32 {
                matches = 1;
                prev_index = line_filter_index as i32;
                continue;
            }
            else {
                matches = matches + 1;
            }
            if matches == self.min_req_consequent_matches {
                matches = 0;
                line_filter_indexes_with_min_req_matches.push(line_filter_index as u32);
                last_inserted_index = line_filter_index as i32;
            }
        }
        return line_filter_indexes_with_min_req_matches;
    }

    fn _find_best_matching_filter_index(&self, words: &Vec<String>) -> i32 {
        if self.line_filters.len() == 0 {
            return -1
        }

        let mut best_matching_filter_index: i32 = -1;
        let mut max_consequent_matches = 0;
        for filter_index in self._get_line_filter_indexes_with_min_req_matches(words) {
            let max_cur_consequent_matches = self._count_consequent_matches_in_line_filter(words, filter_index);
            if max_cur_consequent_matches > max_consequent_matches {
                max_consequent_matches = max_cur_consequent_matches;
                best_matching_filter_index = filter_index as i32;
            }
        }
        if max_consequent_matches > self.min_req_consequent_matches {
            return best_matching_filter_index;
        }
        return -1;
    }

    fn _add_to_filters(&mut self, log_line: &str) {
        let words_iterator = log_line.split(|c|
            c == ' ' ||
            c == '/' ||
            c == ',' ||
            c == '.' ||
            c == ':' ||
            c == '"' ||
            c == '(' ||
            c == ')' ||
            c == '{' ||
            c == '}' ||
            c == '[' ||
            c == ']');
        let mut words = Vec::new();

        for word in words_iterator {
            let word = word.to_string();
            if word.len() > 0 {
                words.push(word);
            }
        }

        let matched_filter_index = self._find_best_matching_filter_index(&words);
        if matched_filter_index >= 0 {
            // TODO (add alternative words)
        }
        else {
            let mut words_alternatives = Vec::new();
            let expected_index = self.line_filters.len() as u32;

            for word in words {
                if word.len() > 0 {
                    self._update_hash(&word, expected_index);
                    words_alternatives.push(vec![word]);
                }
            }
            self.line_filters.push(words_alternatives);
        }
    }

    fn learn_line(&mut self, log_line: &str) {
        self._add_to_filters(log_line);
    }

    fn save_filters(self) {
        // TODO
    }

    fn load_filters(self) {
        // TODO
    }

    fn print(self) {
        if self.line_filters.len() > 0 {
            for elem in self.line_filters {
                println!("{:?}", elem);
            }
        }
        else {
            println!("No filters added yet");
        }
        println!();
        if self.words_hash.len() > 0 {
            for (key, value) in self.words_hash {
                println!("{} : {:?}", key, value);
            }
        }
        else {
            println!("No words with references to filters added yet");
        }
    }
}

fn main() {
    let std_in = io::stdin();
    let mut log_filters = LogFilters::new();

    let mut icnt = 0;
    for line in std_in.lock().lines() {
        let log_line = line.expect("INVALID INPUT!");
        log_filters.learn_line(&log_line);
        icnt += 1;
        if icnt % 1000 == 0 {
            eprintln!("{}", icnt);
        }
    }

    log_filters.print();
}
