use crate::operators;
use opticore_core::objective::ObjectiveStatus;
use opticore_core::Objective;
use rand::random;

/// This first algorithm is just a starting point for implementation.
/// The goal is to use it as a practical example to set up the repository structure,
/// along with the different structs and methods.
/// Once that is in place, we will focus on improving and adding more algorithms.

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
        self._seed = seed
    }
}

pub struct LocalSearch<CostFunction>
where
    CostFunction: Fn(&Vec<usize>) -> f64,
{
    objective: Objective<CostFunction>,
    parameters: LocalSearchParameters,
    solution: Vec<usize>,
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

    pub fn solve(&mut self) -> ObjectiveStatus {
        let mut n_iterations: usize = 0;

        while n_iterations < self.parameters.iterations_limit {
            n_iterations += 1;

            let neighbour = operators::two_opt_swap(self.solution());
            let cost = self.objective.evaluate(&neighbour);

            if self.objective.is_better(cost, self.objective.value()) {
                self.objective.update(&neighbour);
                self.solution = neighbour;
            };
        }
        self.objective.status()
    }
}
