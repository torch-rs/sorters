extern crate regex;
extern crate search_candidate;

use self::search_candidate::SearchCandidate;
use self::search_candidate::Key;
use std::cmp::Ordering;
use std::path::MAIN_SEPARATOR;

use self::regex::Regex;

use Sort;

pub struct FilesSorter;

impl Sort for FilesSorter {

    fn sort(candidates: &Vec<SearchCandidate>) -> Vec<SearchCandidate> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| {
            let hidden_file_pattern = Regex::new(&regex::escape(&format!("{}.", MAIN_SEPARATOR))).unwrap();
            let a_display_text = a.get_value(Key::DisplayText);
            let b_display_text = b.get_value(Key::DisplayText);
            if hidden_file_pattern.is_match(&a_display_text) && hidden_file_pattern.is_match(&b_display_text) {
                return a_display_text.cmp(&b_display_text);
            } else if hidden_file_pattern.is_match(&a_display_text) {
                return Ordering::Greater;
            } else if hidden_file_pattern.is_match(&b_display_text) {
                return Ordering::Less;
            } else {
                return a_display_text.cmp(&b_display_text);
            }
        });
        candidates
    }    
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Sort;
    use files_sorter::FilesSorter;
    use self::search_candidate::SearchCandidate;
    use self::search_candidate::Key;

    #[test]
    fn test_sort() {
        let sample_search_candidates = vec![
            SearchCandidate::new(String::from("/home/test/nothidden"),
                                 String::from("/home/test/nothidden"), String::new()),
            SearchCandidate::new(String::from("/home/test/.hidden"),
                                 String::from("/home/test/.hidden"), String::new()),
            SearchCandidate::new(String::from("/home/test/.hiddentwo"),
                                 String::from("/home/test/.hiddentwo"), String::new()),
            SearchCandidate::new(String::from("/home/test/.hiddenthree"),
                                 String::from("/home/test/.hiddenthree"), String::new()),
            SearchCandidate::new(String::from("/home/test/nothiddentwo"),
                                 String::from("/home/test/nothiddentwo"), String::new()),
        ];
        let actual_sorted_order = vec!["/home/test/nothidden", "/home/test/nothiddentwo",
                                       "/home/test/.hidden", "/home/test/.hiddenthree",
                                       "/home/test/.hiddentwo"];
        let sorted_candidates = FilesSorter::sort(&sample_search_candidates); 
        for i in 0..sorted_candidates.len() {
            assert_eq!(sorted_candidates[i].get_value(Key::DisplayText), actual_sorted_order[i]);
        }
    }
}
