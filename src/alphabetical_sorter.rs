use Sorter;

pub struct AlphabeticalSorter;

impl Sorter for AlphabeticalSorter {

    fn sort(candidates: &Vec<String>) -> Vec<String> {
        let mut candidates = candidates.to_vec();
        candidates.sort_by(|a, b| a.cmp(b));
        candidates
    }

}

#[cfg(test)]
mod tests {

    use Sorter;
    use alphabetical_sorter::AlphabeticalSorter;

    #[test]
    fn test_sort() {
        let test_vec = vec!["/home/test/nothidden".to_string(), "/home/test/.hidden".to_string(),
                            "/home/test/.hiddentwo".to_string(), "/home/test/.hiddenthree".to_string(),
                            "/home/test/nothiddentwo".to_string()];
        assert_eq!(AlphabeticalSorter::sort(&test_vec), ["/home/test/.hidden", "/home/test/.hiddenthree",
                                                         "/home/test/.hiddentwo", "/home/test/nothidden",
                                                         "/home/test/nothiddentwo"]);
    }
}
