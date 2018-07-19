extern crate fs2;
extern crate serde;
extern crate serde_pickle;

use self::fs2::FileExt;

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

    fn sort(&self, candidates: &Vec<String>) -> Vec<String> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| {
            let weight_a = self.weights.get(a.as_str()).unwrap_or(&1);
            let weight_b = self.weights.get(b.as_str()).unwrap_or(&1);
            if weight_a < weight_b {
                return Ordering::Greater;
            } else if weight_a > weight_b {
                return Ordering::Less;
            } else {
                return a.cmp(b);
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
        if let Some(value) = self.weights.get_mut(&key) {
            *value -= 1;
        }
    }

    fn increment_weight(&mut self, key: String) {
        if let Some(value) = self.weights.get_mut(&key) {
            *value += 1;
        }
    }

    fn update_weight(&mut self, key: String, value: i32) {
        self.weights.insert(key, value);
    }
    
}

#[cfg(test)]
mod tests {

    use WeightedSort;
    use frequency_sorter::FrequencySorter;
    use std::fs::remove_file;

    #[test]
    fn no_file_create() {
        let sorter = FrequencySorter::new("app".to_string());
        let sample_data = vec!["abc".to_string(), "adc".to_string(), "aaa".to_string(), "bbvb".to_string(),
                               "bacs".to_string()];
        let sorted_data = sorter.sort(&sample_data);
        assert_eq!(sorted_data, ["aaa", "abc", "adc", "bacs", "bbvb"]);
    }

    #[test]
    fn with_file_created() {
        let mut setup_sorter = FrequencySorter::new("test".to_string());
        setup_sorter.update_weight(String::from("bacs"), 4);
        setup_sorter.update_weight(String::from("adc"), 5);
        setup_sorter.save();

        let sample_data = vec!["abc".to_string(), "adc".to_string(), "aaa".to_string(), "bbvb".to_string(),
                               "bacs".to_string()];
        let sorter = FrequencySorter::new("test".to_string());
        let sorted_data = sorter.sort(&sample_data);
        assert_eq!(sorted_data, ["adc", "bacs", "aaa", "abc", "bbvb"]);
        remove_file("test").unwrap();
    }

}
