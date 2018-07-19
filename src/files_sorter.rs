extern crate regex;

use std::cmp::Ordering;
use std::path::MAIN_SEPARATOR;

use self::regex::Regex;

use Sort;

pub struct FilesSorter;

impl Sort for FilesSorter {

    fn sort(candidates: &Vec<String>) -> Vec<String> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| {
            let hidden_file_pattern = Regex::new(&regex::escape(&format!("{}.", MAIN_SEPARATOR))).unwrap();
            if hidden_file_pattern.is_match(a) && hidden_file_pattern.is_match(b) {
                return a.cmp(b);
            } else if hidden_file_pattern.is_match(a) {
                return Ordering::Greater;
            }else if hidden_file_pattern.is_match(b) {
                return Ordering::Less;
            } else {
                return a.cmp(b);
            }
        });
        candidates
    }    
    
}

#[cfg(test)]
mod tests {

    use Sort;
    use files_sorter::FilesSorter;

    #[test]
    fn test_sort() {
        let test_vec = vec!["/home/test/nothidden".to_string(), "/home/test/.hidden".to_string(),
                            "/home/test/.hiddentwo".to_string(), "/home/test/.hiddenthree".to_string(),
                            "/home/test/nothiddentwo".to_string()];
        assert_eq!(FilesSorter::sort(&test_vec), ["/home/test/nothidden", "/home/test/nothiddentwo",
                                                  "/home/test/.hidden", "/home/test/.hiddenthree",
                                                  "/home/test/.hiddentwo"]);
    }
}
