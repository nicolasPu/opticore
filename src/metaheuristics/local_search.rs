use crate::metaheuristics::{Objective, ObjectiveType, Solution};
use rand::Rng;

const SEED_LIMIT: usize = 1000;

// todo:
// implement the seed

// Implement a neighborhood instead of the neighbour
// Implement a custom neighborhood generator for the user

// Implement feasibility as a new func?
// do we need lifespans?

// implement the time limit // timeout on sync function? rayon and threats?

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

#[derive(Copy, Clone)] // remove once time limit is implemented
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

pub struct LocalSearch<EvaluatorFunction>
where
    EvaluatorFunction: Fn(&Vec<usize>) -> f64,
{
    objective: Objective<EvaluatorFunction>,
    parameters: LocalSearchParameters,
    pub solution: Solution,
}

impl<EvaluatorFunction> LocalSearch<EvaluatorFunction>
where
    EvaluatorFunction: Fn(&Vec<usize>) -> f64,
{
    pub fn new(
        initial_state: Vec<usize>,
        objective: Objective<EvaluatorFunction>,
        parameters: LocalSearchParameters,
    ) -> Self {
        // Should we handle errors here?
        let initial_cost_value = objective.evaluate(&initial_state);

        Self {
            parameters,
            objective,
            solution: Solution::new_feasible(initial_state, initial_cost_value),
        }
    }

    pub fn solution(&self) -> Vec<usize> {
        self.solution.state.clone()
    }

    pub fn objective_value(&self) -> f64 {
        self.solution.objective_value.clone()
    }

    pub fn solve(&mut self) -> String {
        let mut n_iterations: usize = 0;

        while n_iterations < self.parameters.iterations_limit {
            n_iterations += 1;
            let neighbour = two_opt_swap(self.solution.state.clone());
            let cost = self.objective.evaluate(&neighbour);
            let neighbour_solution = Solution::new_feasible(neighbour, cost);

            // This can be improved -- implement struct comparison and min max cases
            match self.objective.goal {
                ObjectiveType::Max => {
                    if neighbour_solution.objective_value > self.solution.objective_value {
                        self.solution = neighbour_solution;
                    }
                }
                _ => {
                    if neighbour_solution.objective_value < self.solution.objective_value {
                        self.solution = neighbour_solution;
                    }
                }
            };
        }
        self.solution.string_status()
    }
}
