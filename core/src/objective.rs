use serde::{Deserialize, Serialize};

//Constrain satisfaction objective?
pub enum ObjectiveType {
    Max,
    Min,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
enum ObjectiveStatus {
    Feasible,
    Optimal,
    Unfeasible,
    Unbounded,
    Unknown,
}

pub struct Objective<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    cost_function: CostFunction,
    pub goal: ObjectiveType,
    pub objective_value: f64,
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

    pub fn update(&mut self, solution: &Vec<usize>) {
        self.objective_value = self.evaluate(solution);
        self.status = ObjectiveStatus::Feasible;
    }

    pub fn evaluate(&self, solution: &Vec<usize>) -> f64 {
        (self.cost_function)(solution)
    }

    pub fn status(&self) -> String {
        serde_json::to_string(&self.status).unwrap_or_else(|_| "Unknown".to_string())
    }
}
