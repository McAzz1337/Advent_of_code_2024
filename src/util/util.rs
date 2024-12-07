pub type Grid = Vec<Vec<char>>;

pub fn to_matrix(v: &Vec<String>) -> Grid {
    v.iter().map(|s| s.chars().collect()).collect()
}
