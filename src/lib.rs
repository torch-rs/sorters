pub mod alphabetical_sorter;
pub mod files_sorter;

pub trait Sort {

    fn sort(candidates: &Vec<String>) -> Vec<String>;

}

#[cfg(test)]
mod tests {

}
