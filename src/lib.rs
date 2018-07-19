pub mod alphabetical_sorter;
pub mod files_sorter;

pub trait Sort {

    fn sort(candidates: &Vec<String>) -> Vec<String>;

}

pub trait WeightedSort {

    fn decrement_weight(&mut self, key: String);
    fn increment_weight(&mut self, key: String);
    fn new(filename: String) -> Self;
    fn save(&self);
    fn sort(&self, candidates: &Vec<String>) -> Vec<String> ;
    fn update_weight(&mut self, key: String, value: i32);

}

#[cfg(test)]
mod tests {

}
