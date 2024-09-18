use serde::{Deserialize, Serialize};

pub mod local_search;

// do we want to implement a pre-solve to check unbounded or infeasible?
// it would be very cool to implement a theoretical OF gap
// handling errors for evaluate function

pub enum ObjectiveType {
    Max,
    Min,
    //Satisfiability,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
enum ObjectiveStatus {
    Optimal,
    Feasible,
    UnFeasible,
    Unbounded,
    Unknown,
}

pub struct Objective<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    cost_function: CostFunction,
    goal: ObjectiveType,
    objective_value: f64,
    status: ObjectiveStatus,
}

impl<CostFunction> Objective<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    pub fn new(cost_function: CostFunction, goal: ObjectiveType) -> Self {
        let initial_objective_function = match goal {
            ObjectiveType::Min => f64::INFINITY,
            ObjectiveType::Max => -f64::INFINITY,
        };
        Self {
            cost_function,
            goal,
            objective_value: initial_objective_function,
            status: ObjectiveStatus::Unknown,
        }
    }

    fn update(&mut self, solution: &Vec<usize>) {
        self.objective_value = self.evaluate(solution);
        self.status = ObjectiveStatus::Feasible;
    }

    fn evaluate(&self, status: &Vec<usize>) -> f64 {
        (self.cost_function)(status)
    }

    fn status(&self) -> String {
        serde_json::to_string(&self.status).unwrap_or_else(|_| "Unknown".to_string())
    }
}
