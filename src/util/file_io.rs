use std::fs;

fn read_file(file_name: String) -> Vec<String> {
    let file = fs::read_to_string(file_name).unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn get_input_path(day: u32, test: bool) -> String {
    let path = if test {
        String::from("input/test/day")
    } else {
        String::from("input/day")
    };
    let day = day.to_string();
    let day = day.as_str();
    path + day + ".txt"
}

fn get_input_path_part(day: u32, part: u32, test: bool) -> String {
    let path = if test {
        String::from("input/test/day")
    } else {
        String::from("input/day")
    };
    let day = day.to_string();
    let day = day.as_str();
    let part = String::from("_part") + part.to_string().as_str();
    path + day + part.as_str() + ".txt"
}

pub fn get_input(day: u32) -> Vec<String> {
    read_file(get_input_path(day, false))
}

pub fn get_input_part(day: u32, part: u32) -> Vec<String> {
    read_file(get_input_path_part(day, part, false))
}

pub fn get_test_input(day: u32) -> Vec<String> {
    read_file(get_input_path(day, true))
}

pub fn get_test_input_part(day: u32, part: u32) -> Vec<String> {
    read_file(get_input_path_part(day, part, true))
}
