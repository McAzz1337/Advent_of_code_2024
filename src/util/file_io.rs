use std::fs;

fn read_file(file_name: String) -> Vec<String> {
    let file = fs::read_to_string(file_name).unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn get_input_path(day: i32, test: bool) -> String {
    let path = if test {
        String::from("input/test/day")
    } else {
        String::from("input/day")
    };
    let day = day.to_string();
    let day = day.as_str();
    path + day + ".txt"
}

pub fn get_input(day: i32) -> Vec<String> {
    read_file(get_input_path(day, false))
}
pub fn get_test_input(day: i32) -> Vec<String> {
    read_file(get_input_path(day, true))
}
