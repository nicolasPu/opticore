use std::marker::PhantomData;

pub enum ObjectiveType {
    Max,
    Min,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectiveStatus {
    Feasible,
    Optimal,
    Unfeasible,
    Unbounded,
    Unknown,
}

pub struct Objective<Solution, CostFn>
where
    CostFn: Fn(&Solution) -> f64,
{
    cost_function: CostFn,
    goal: ObjectiveType,
    objective_value: f64,
    status: ObjectiveStatus,
    _marker: PhantomData<Solution>, // tells the compiler this Objective is generic with Solution
}

impl<Solution, CostFn> Objective<Solution, CostFn>
where
    CostFn: Fn(&Solution) -> f64,
{
    pub fn new(cost_function: CostFn, goal: ObjectiveType) -> Self {
        let initial_objective_function = match goal {
            ObjectiveType::Min => f64::INFINITY,
            ObjectiveType::Max => -f64::INFINITY,
        };
        Self {
            cost_function,
            goal,
            objective_value: initial_objective_function,
            status: ObjectiveStatus::Unknown,
            _marker: PhantomData,
        }
    }

    pub fn is_better(&self, new_cost: f64, old_cost: f64) -> bool {
        match self.goal {
            ObjectiveType::Max => new_cost > old_cost,
            ObjectiveType::Min => new_cost < old_cost,
        }
    }

    pub fn evaluate(&self, solution: &Solution) -> f64 {
        (self.cost_function)(solution)
    }

    pub fn get_status(&self) -> ObjectiveStatus {
        self.status.clone()
    }

    pub fn current_value(&self) -> f64 {
        self.objective_value.clone()
    }

    pub fn update(&mut self, solution: &Solution) {
        self.objective_value = self.evaluate(solution);
        self.status = ObjectiveStatus::Feasible;
    }
}
