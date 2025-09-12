use opticore::objective::{Objective, ObjectiveType};
use opticore::metaheuristics::local_search::{LocalSearch, LocalSearchParameters};

const NUM_WORKERS: usize = 3;
const NUM_TASKS: usize = 4;

const COST: [[f64; NUM_TASKS]; NUM_WORKERS] = [
    [12., 18., 25., 30.],
    [40., 22., 15., 10.],
    [33., 27., 19., 21.],
];

fn main() {
    // You can also use a closure for the objective function
    let cost_function = |solution: &Vec<usize> | -> f64 {
        solution
            .iter()
            .enumerate()
            .map(|(task, &worker)| COST[worker][task])
            .sum()
    };

    // Define the objective function
    let objective = Objective::new(cost_function, ObjectiveType::Min);

    // Initial feasible solution (all task for the first worker)
    let initial_state: Vec<usize> = (0..NUM_TASKS)
        .map(|task| task % NUM_WORKERS)
        .collect();

    // Local search algorithm
    let parameters = LocalSearchParameters::default();
    let mut solver = LocalSearch::new(initial_state, objective, parameters);

    // Solve
    let status = solver.solve();

    // Print solution
    println!("Status: {:?}", status);
    println!("Best cost: {}", solver.best_value());
    println!("Best route: {:?}", solver.current_solution());
}

