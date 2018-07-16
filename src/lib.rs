pub mod files_sorter;

pub trait Sorter {

    fn sort(candidates: &Vec<String>) -> Vec<String>;

}

#[cfg(test)]
mod tests {

}
