use std::fmt::Display;

pub struct PuzzleResult<T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
{
    day: i32,
    result_part_1: Option<T>,
    result_part_2: Option<U>,
}

impl<T, U> PuzzleResult<T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
{
    pub fn new(day: i32) -> PuzzleResult<T, U> {
        PuzzleResult {
            day,
            result_part_1: None,
            result_part_2: None,
        }
    }

    pub fn result_part_1(&mut self, result: T) {
        self.result_part_1 = Some(result);
    }

    pub fn result_part_2(&mut self, result: U) {
        self.result_part_2 = Some(result);
    }
}

impl<T, U> Display for PuzzleResult<T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result1 = self.result_part_1.clone().map_or_else(
            || "PART 1: TO BE SOLVED".to_string(),
            |x| format!("PART 1: {}", x.to_string()),
        );
        let result2 = self.result_part_2.clone().map_or_else(
            || "PART 2: TO BE SOLVED".to_string(),
            |x| format!("PART 2: {}", x.to_string()),
        );
        let output = String::from("-------DAY ")
            + self.day.to_string().as_str()
            + "-------\n\t"
            + result1.as_str()
            + "\n\t"
            + result2.as_str();

        f.write_str(&output)
    }
}
