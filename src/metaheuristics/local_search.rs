use crate::objective::{Objective, ObjectiveStatus};
use crate::operators;
use rand::random;

#[allow(dead_code)]
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
            _seed: seed.unwrap_or_else(|| random()),
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
        self._seed = seed
    }
}

pub struct LocalSearch<CostFn>
where
    CostFn: Fn(&Vec<usize>) -> f64,
{
    objective: Objective<Vec<usize>, CostFn>,
    parameters: LocalSearchParameters,
    solution: Vec<usize>,
}

impl<CostFn> LocalSearch<CostFn>
where
    CostFn: Fn(&Vec<usize>) -> f64,
{
    pub fn new(
        initial_state: Vec<usize>,
        objective: Objective<Vec<usize>, CostFn>,
        parameters: LocalSearchParameters,
    ) -> Self {
        Self {
            parameters,
            objective,
            solution: initial_state,
        }
    }

    pub fn solve(&mut self) -> ObjectiveStatus {
        let mut n_iterations: usize = 0;

        while n_iterations < self.parameters.iterations_limit {
            n_iterations += 1;

            let neighbour = operators::two_opt_swap(self.current_solution());
            let cost = self.objective.evaluate(&neighbour);

            if self
                .objective
                .is_better(cost, self.objective.current_value())
            {
                self.objective.update(&neighbour);
                self.solution = neighbour;
            }
        }

        self.objective.get_status()
    }

    pub fn current_solution(&self) -> Vec<usize> {
        self.solution.clone()
    }

    pub fn best_value(&self) -> f64 {
        self.objective.current_value()
    }
}
