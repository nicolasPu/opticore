use crate::metaheuristics::{Objective, ObjectiveType, Solution, SolutionStatus};
use rand::Rng;
use std::time::Duration;

const SEED_LIMIT: usize = 1000;

// todo:
// Implement feasibility as a new func?
// Implement a neighborhood instead of the neighbour
// Implement a custom neighborhood generator
// do we need lifespans?
// implement the time limit
// implement the seed


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

#[allow(dead_code)] // remove once time limit is implemented
pub struct LocalSearchParameters {
    iterations_limit: usize,
    time_limit: Duration,
    seed: usize,
}

impl LocalSearchParameters {
    pub fn new(iterations_limit: usize, time_limit: Duration, seed: Option<usize>) -> Self {
        Self {
            iterations_limit,
            time_limit,
            seed: seed.unwrap_or_else(|| {
                rand::thread_rng().gen_range(0..SEED_LIMIT) // can we try negatives?
            }),
        }
    }
}


pub struct LocalSearch<F>
where
    F: Fn(&Vec<usize>) -> f64,
{
    initial_state: Vec<usize>,
    parameters: LocalSearchParameters,
    objective: Objective<F>,
}

impl <F>LocalSearch<F>
where
    F: Fn(&Vec<usize>) -> f64,
{
    pub fn new(
        evaluation_function: F,
        objective_type: ObjectiveType, // do we want to pass the objective struct? like with the params
        initial_state: Vec<usize>,
        parameters: LocalSearchParameters,
    ) -> Self {
        assert!(
            initial_state.len() > 1,
            "Initial state must have at least two elements"
        );

        Self {
            initial_state,
            parameters,
            objective: Objective::new(evaluation_function, objective_type),
        }
    }

    pub fn solve(&self) -> Solution {
        let mut best_state = self.initial_state.clone();
        let mut current_objective_value = self.objective.evaluate(&best_state);
        let mut n_iterations: usize = 0;
        while n_iterations < self.parameters.iterations_limit {
            n_iterations += 1;
            let neighbour = two_opt_swap(best_state.clone());
            let neighbour_objective_value = self.objective.evaluate(&neighbour);

            match self.objective.goal{
                ObjectiveType::Min => {
                    if neighbour_objective_value < current_objective_value {
                        current_objective_value = neighbour_objective_value;
                        best_state = neighbour;
                    }
                },
                _ => {
                    if neighbour_objective_value > current_objective_value {
                        current_objective_value = neighbour_objective_value;
                        best_state = neighbour;
                    }
                }
            };
        }
        let solution = Solution {
            objective_value: current_objective_value,
            solution_state: best_state,
            status: SolutionStatus::Feasible,
        };

        solution
    }
}
