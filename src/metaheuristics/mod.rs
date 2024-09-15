pub mod local_search;

// do we want to implement a pre-solve to check unbounded or infeasible?
// it would be very cool to implement a theoretical OF gap
// handling errors for evaluate function

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum SolutionStatus {
    Optimal,
    Feasible,
    UnFeasible,
    Unbounded,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct Solution {
    objective_value: f64,
    state: Vec<usize>,
    status: SolutionStatus,
}

impl Solution {
    pub fn new_feasible(state: Vec<usize>, objective_value: f64) -> Self {
        Self {
            objective_value,
            state,
            status: SolutionStatus::Feasible,
        }
    }

    pub fn get_solution(&self) -> Vec<usize> {
        self.state.clone()
    }

    pub fn report(&self) {
        println!("Objective Value: {}", self.objective_value);
        println!("Status: {:?}", self.status);
    }
}

pub enum ObjectiveType {
    Max,
    Min,
    Satisfiability,
}

pub struct Objective<F>
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
    pub fn new(evaluate_function: F, goal: ObjectiveType) -> Self {
        Self {
            evaluate_function,
            goal,
        }
    }

    fn evaluate(&self, status: &Vec<usize>) -> f64 {
        (self.evaluate_function)(status)
    }
}
