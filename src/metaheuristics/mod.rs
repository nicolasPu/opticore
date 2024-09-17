use serde::{Deserialize, Serialize};

pub mod local_search;

// do we want to implement a pre-solve to check unbounded or infeasible?
// it would be very cool to implement a theoretical OF gap
// handling errors for evaluate function

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
enum SolutionStatus {
    Optimal,
    Feasible,
    UnFeasible,
    Unbounded,
    Unknown,
}
#[derive(Clone)]
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
    
    pub fn string_status(&self) -> String {
        serde_json::to_string(&self.status).unwrap_or_else(|_| "Unknown".to_string())
    } 
}

pub enum ObjectiveType {
    Max,
    Min,
    Satisfiability,
}

pub struct Objective<EvaluatorFunction>
where
    EvaluatorFunction: Fn(&Vec<usize>) -> f64,
{
    evaluate_function: EvaluatorFunction,
    goal: ObjectiveType,
}

impl<EvaluatorFunction> Objective<EvaluatorFunction>
where
    EvaluatorFunction: Fn(&Vec<usize>) -> f64,
{
    pub fn new(evaluate_function: EvaluatorFunction, goal: ObjectiveType) -> Self {
        Self {
            evaluate_function,
            goal,
        }
    }

    fn evaluate(&self, status: &Vec<usize>) -> f64 {
        (self.evaluate_function)(status)
    }
}
