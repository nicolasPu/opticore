use crate::metaheuristics::{Objective, ObjectiveType};
use rand::Rng;

const SEED_LIMIT: usize = 1000;

// todo:
// Implement a neighborhood instead of the neighbour
// handling errors for evaluate function

fn two_opt_swap(mut state: Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let vec_len = state.len();

    let index1 = rng.gen_range(0..vec_len);
    let mut index2 = rng.gen_range(0..vec_len);

    while index1 == index2 {
        index2 = rng.gen_range(0..vec_len);
    }

    state.swap(index1, index2);

    state
}

pub struct LocalSearchParameters {
    iterations_limit: usize,
    _seed: usize,
}

impl LocalSearchParameters {
    pub fn new(iterations_limit: usize, seed: Option<usize>) -> Self {
        Self {
            iterations_limit,
            _seed: seed.unwrap_or_else(|| {
                rand::thread_rng().gen_range(0..SEED_LIMIT) // can we try negatives?
            }),
        }
    }
}

pub struct LocalSearch<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    objective: Objective<CostFunction>,
    parameters: LocalSearchParameters,
    pub solution: Vec<usize>,
}

impl<CostFunction> LocalSearch<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    pub fn new(
        initial_state: Vec<usize>,
        objective: Objective<CostFunction>,
        parameters: LocalSearchParameters,
    ) -> Self {
        // Should we handle feasibility errors here?
        Self {
            parameters,
            objective,
            solution: initial_state,
        }
    }

    pub fn solution(&self) -> Vec<usize> {
        self.solution.clone()
    }

    pub fn objective_value(&self) -> f64 {
        self.objective.objective_value.clone()
    }

    pub fn solve(&mut self) -> String {
        let mut n_iterations: usize = 0;

        while n_iterations < self.parameters.iterations_limit {
            n_iterations += 1;
            let neighbour = two_opt_swap(self.solution.clone());
            let cost = self.objective.evaluate(&neighbour);
            let should_update = match self.objective.goal {
                ObjectiveType::Max => cost > self.objective.objective_value,
                ObjectiveType::Min => cost < self.objective.objective_value,
            };
            if should_update {
                self.objective.update(&neighbour);
                self.solution = neighbour;
            };
        }
        self.objective.status()
    }
}
