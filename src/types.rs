pub trait Solver {
    fn solve(&self, problem_input: String) -> Result<String, ProcessError>;
}
#[derive(Debug)]
pub struct ProcessError {}

pub struct DataRetreiver {}
