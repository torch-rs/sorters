extern crate search_candidate;

pub mod alphabetical_sorter;
pub mod files_sorter;
pub mod frequency_sorter;

use self::search_candidate::SearchCandidate;

pub trait Sort {

    fn sort(candidates: &Vec<SearchCandidate>) -> Vec<SearchCandidate>;

}

pub trait WeightedSort {

    fn decrement_weight(&mut self, key: String);
    fn increment_weight(&mut self, key: String);
    fn new(filename: String) -> Self;
    fn save(&self);
    fn sort(&self, candidates: &Vec<SearchCandidate>) -> Vec<SearchCandidate> ;
    fn update_weight(&mut self, key: String, value: i32);

}

#[cfg(test)]
mod tests {

}
