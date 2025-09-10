use serde::{Deserialize, Serialize};

//Constrain satisfaction objective?
pub enum ObjectiveType {
    Max,
    Min,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ObjectiveStatus {
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

    pub fn is_better(&self, new_cost: f64, old_cost: f64) -> bool {
        match self.goal {
            ObjectiveType::Max => new_cost > old_cost,
            ObjectiveType::Min => new_cost < old_cost,
        }
    }

    pub fn evaluate(&self, solution: &Vec<usize>) -> f64 {
        (self.cost_function)(solution)
    }

    pub fn status(&self) -> ObjectiveStatus {
        self.status.clone()
    }

    pub fn value(&self) -> f64 {
        self.objective_value.clone()
    }

    pub fn update(&mut self, solution: &Vec<usize>) {
        self.objective_value = self.evaluate(solution);
        self.status = ObjectiveStatus::Feasible;
    }
}
