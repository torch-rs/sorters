extern crate fs2;
extern crate search_candidate;
extern crate serde;
extern crate serde_pickle;

use self::fs2::FileExt;
use self::search_candidate::Key;
use self::search_candidate::SearchCandidate;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use WeightedSort;

pub struct FrequencySorter {
    filename: String,
    weights: HashMap<String, i32>,
}

impl WeightedSort for FrequencySorter {

    fn new(filename: String) -> Self {
        match File::open(filename.clone()) {
            Ok(mut file) => {
                file.lock_exclusive().unwrap();
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).expect("Something went wrong while reading the file");
                file.unlock().unwrap();
                return FrequencySorter {
                    filename: filename,
                    weights: serde_pickle::from_slice(&contents).unwrap(),
                }
            }
            Err(_e) => {
                return FrequencySorter {
                    filename: filename,
                    weights: HashMap::new(),
                }
            }
        }
    }

    fn sort(&self, candidates: &Vec<SearchCandidate>) -> Vec<SearchCandidate> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| {
            let a_display_text = a.get_value(Key::DisplayText);
            let b_display_text = b.get_value(Key::DisplayText);
            let weight_a = self.weights.get(a_display_text.as_str()).unwrap_or(&1);
            let weight_b = self.weights.get(b_display_text.as_str()).unwrap_or(&1);
            if weight_a < weight_b {
                return Ordering::Greater;
            } else if weight_a > weight_b {
                return Ordering::Less;
            } else {
                return a_display_text.cmp(&b_display_text);
            }
        });
        candidates
    }

    fn save(&self) {
        let serialized = serde_pickle::to_vec(&self.weights, true).unwrap();
        let mut file = File::create(self.filename.clone()).unwrap();
        file.lock_exclusive().unwrap();
        if file.write_all(&serialized[..]).is_err() {
            eprintln!("Couldn't save to file");
        }
        file.unlock().unwrap();
    }

    fn decrement_weight(&mut self, key: String) {
        *self.weights.entry(key).or_insert(1) -= 1;
    }

    fn increment_weight(&mut self, key: String) {
        *self.weights.entry(key).or_insert(1) += 1;
    }

    fn update_weight(&mut self, key: String, value: i32) {
        self.weights.insert(key, value);
    }
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use frequency_sorter::FrequencySorter;
    use self::search_candidate::Key;
    use self::search_candidate::SearchCandidate;
    use std::fs::remove_file;
    use WeightedSort;

    #[test]
    fn no_file_create() {
        let sorter = FrequencySorter::new("test".to_string());
        let sample_search_candidates = vec![
            SearchCandidate::new(String::from("abc"),
                                 String::from("abc"), String::new()),
            SearchCandidate::new(String::from("adc"),
                                 String::from("adc"), String::new()),
            SearchCandidate::new(String::from("aaa"),
                                 String::from("aaa"), String::new()),
            SearchCandidate::new(String::from("bbvb"),
                                 String::from("bbvb"), String::new()),
            SearchCandidate::new(String::from("bacs"),
                                 String::from("bacs"), String::new()),
        ];
        let actual_sorted_order = vec!["aaa", "abc", "adc", "bacs", "bbvb"];
        let sorted_candidates = sorter.sort(&sample_search_candidates); 
        for i in 0..sorted_candidates.len() {
            assert_eq!(sorted_candidates[i].get_value(Key::DisplayText), actual_sorted_order[i]);
        }
    }

    #[test]
    fn with_file_created() {
        let mut setup_sorter = FrequencySorter::new("another_test".to_string());
        setup_sorter.update_weight(String::from("bacs"), 4);
        setup_sorter.update_weight(String::from("adc"), 5);
        setup_sorter.save();

        let sorter = FrequencySorter::new("another_test".to_string());
        let sample_search_candidates = vec![
            SearchCandidate::new(String::from("abc"),
                                 String::from("abc"), String::new()),
            SearchCandidate::new(String::from("adc"),
                                 String::from("adc"), String::new()),
            SearchCandidate::new(String::from("aaa"),
                                 String::from("aaa"), String::new()),
            SearchCandidate::new(String::from("bbvb"),
                                 String::from("bbvb"), String::new()),
            SearchCandidate::new(String::from("bacs"),
                                 String::from("bacs"), String::new()),
        ];
        let actual_sorted_order = vec!["adc", "bacs", "aaa", "abc", "bbvb"];
        let sorted_candidates = sorter.sort(&sample_search_candidates); 
        for i in 0..sorted_candidates.len() {
            assert_eq!(sorted_candidates[i].get_value(Key::DisplayText), actual_sorted_order[i]);
        }
        remove_file("another_test").unwrap();
    }

}
