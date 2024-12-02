use std::fs;

pub fn read_file(file_name: String) -> Vec<String> {
    let file = fs::read_to_string(file_name).unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn get_input(day: &str, test: bool) -> Vec<String> {
    let path = if test {
        String::from("input/test/")
    } else {
        String::from("input/")
    };
    read_file(path + day + ".txt")
}
