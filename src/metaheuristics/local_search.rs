use crate::metaheuristics::operators;
use crate::metaheuristics::{Objective, ObjectiveType};
use rand::random;

// todo:
// exploration strategy?
// movement selection?
// Implement a neighborhood instead of the neighbour
// handling errors for evaluate function

#[allow(dead_code)] // until use
pub struct LocalSearchParameters {
    iterations_limit: usize,
    number_of_neighbours: usize,
    _seed: f64,
}

impl LocalSearchParameters {
    pub fn new(iterations_limit: usize, number_of_neighbours: usize, seed: Option<f64>) -> Self {
        Self {
            iterations_limit,
            number_of_neighbours,
            _seed: seed.unwrap_or_else(|| random()), // can we try negatives?
        }
    }

    pub fn default() -> Self {
        Self {
            iterations_limit: 1000,
            number_of_neighbours: 10,
            _seed: random(),
        }
    }

    pub fn set_seed(&mut self, seed: f64) {
        self._seed = seed;
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
            let neighbour = operators::two_opt_swap(self.solution.clone());
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
