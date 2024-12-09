use std::fmt::Display;

pub struct PuzzleResult<F, G, T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
    F: Fn(&Vec<String>) -> T,
    G: Fn(&Vec<String>) -> U,
{
    day: i32,
    input: Vec<String>,
    part1: Option<F>,
    part2: Option<G>,
    omitted: bool,
}

impl<F, G, T, U> PuzzleResult<F, G, T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
    F: Fn(&Vec<String>) -> T,
    G: Fn(&Vec<String>) -> U,
{
    pub fn new(
        day: i32,
        input: Vec<String>,
        part1: Option<F>,
        part2: Option<G>,
    ) -> PuzzleResult<F, G, T, U> {
        PuzzleResult {
            day,
            input,
            part1,
            part2,
            omitted: false,
        }
    }

    pub fn omitted(
        day: i32,
        input: Vec<String>,
        part1: Option<F>,
        part2: Option<G>,
    ) -> PuzzleResult<F, G, T, U> {
        PuzzleResult {
            day,
            input,
            part1,
            part2,
            omitted: true,
        }
    }
}

impl<F, G, T, U> Display for PuzzleResult<F, G, T, U>
where
    T: ToString + Clone,
    U: ToString + Clone,
    F: Fn(&Vec<String>) -> T,
    G: Fn(&Vec<String>) -> U,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.omitted {
            let s =
                String::from("-------DAY ") + self.day.to_string().as_str() + "-------\n\tOmitted";
            f.write_str(&s)
        } else {
            let result1 = self.part1.as_ref().map_or_else(
                || "PART 1: TO BE SOLVED".to_string(),
                |f| format!("Part 2 : {}", f(&self.input).to_string()),
            );
            let result2 = self.part2.as_ref().map_or_else(
                || "PART 2: TO BE SOLVED".to_string(),
                |f| format!("PART 2: {}", f(&self.input).to_string()),
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
}
