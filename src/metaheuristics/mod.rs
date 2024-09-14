pub mod local_search;

// todo:
// do we want to implement a pre-solve to check unbounded or infeasible?
// implement a matrix state instead of a vector

pub enum ObjectiveType{
    Max,
    Min,
    Satisfiability,
}

#[allow(dead_code)]
#[derive(Debug)]
enum SolutionStatus {
    Optimal,
    Feasible,
    UnFeasible,
    Unbounded,
    Unknown,
}

// it would be nice to implement a theoretical gap
pub struct Solution {
    objective_value: f64,
    solution_state: Vec<usize>,
    status: SolutionStatus,
}

impl Solution {
    pub fn report(&self) {
        println!("Objective Value: {}", self.objective_value);
        println!("Solution State: {:?}", self.solution_state);
        println!("Status: {:?}", self.status);
    }
}

struct Objective<F>
where
    F: Fn(&Vec<usize>) -> f64,
{
    evaluate_function: F,
    goal: ObjectiveType,
}

impl<F> Objective<F>
where
    F: Fn(&Vec<usize>) -> f64,
{
    fn new(evaluate_function: F, goal: ObjectiveType)-> Self{
        Self{
            evaluate_function,
            goal,
        }
    }

    fn evaluate(&self, status:&Vec<usize>) -> f64{
        (self.evaluate_function)(status)
    }
}
