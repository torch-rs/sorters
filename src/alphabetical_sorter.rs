extern crate search_candidate;

use Sort;
use self::search_candidate::SearchCandidate;
use self::search_candidate::Key;

pub struct AlphabeticalSorter;

impl Sort for AlphabeticalSorter {

    fn sort(candidates: &Vec<SearchCandidate>) -> Vec<SearchCandidate> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| a.get_value(Key::DisplayText).cmp(&b.get_value(Key::DisplayText)));
        candidates
    }

}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Sort;
    use alphabetical_sorter::AlphabeticalSorter;
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
        let actual_sorted_order = vec!["/home/test/.hidden", "/home/test/.hiddenthree",
                                       "/home/test/.hiddentwo", "/home/test/nothidden",
                                       "/home/test/nothiddentwo"];
        let sorted_candidates = AlphabeticalSorter::sort(&sample_search_candidates); 
        for i in 0..sorted_candidates.len() {
            assert_eq!(sorted_candidates[i].get_value(Key::DisplayText), actual_sorted_order[i]);
        }
    }
}
