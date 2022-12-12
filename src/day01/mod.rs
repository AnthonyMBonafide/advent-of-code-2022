use super::types::{ProcessError, Solver};

struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Day01 {}
    }
}

impl Solver for Day01 {
    fn solve(&self, input: String) -> Result<String, ProcessError> {
        let mut max_count: Option<i32> = None;
        let mut current_counter: Option<i32> = None;
        for line in input.lines() {
            let trimmed_line = line.trim();
            println!("{}", line);
            if !trimmed_line.is_empty() {
                // convert to number
                let parsed_number = match trimmed_line.parse::<i32>() {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        return Err(ProcessError {});
                    }
                };
                // add to current count
                match current_counter {
                    Some(i) => current_counter = Some(i + parsed_number),
                    None => current_counter = Some(parsed_number),
                };
                // go to next
                continue;
            } else {
                // compart current count to max
                // set the max of the two to max_count
                if current_counter > max_count {
                    max_count = current_counter;
                }
                // set current count to 0
                current_counter = None;
            }
        }
        Ok(format!("{:?}", max_count))
    }
}

#[cfg(test)]
mod test {
    use super::super::types::*;
    use super::*;
    #[test]
    fn test_simple_input() {
        let input = String::from(
            "1
        2
        3

        4
        5

        6
        ",
        );
        let day01 = Day01::new();
        assert_eq!(day01.solve(input).unwrap(), String::from("9"));
    }
    #[test]
    fn test_negative_numbers() {
        let input = String::from(
            " -1

        -200",
        );
        let day01 = Day01::new();
        assert_eq!(day01.solve(input).unwrap(), String::from("-1"));
    }
}
