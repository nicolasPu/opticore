use opticore::objective::{Objective, ObjectiveType};
use opticore::metaheuristics::local_search::{LocalSearch, LocalSearchParameters};

mod utils;

// Problem parameters
const NUM_CITIES: usize = 4;

const DISTANCES: [[i64; NUM_CITIES]; NUM_CITIES] = [
    [0, 18, 25, 30], // city 0 → {0,1,2,3}
    [40, 0, 15, 10],
    [33, 27, 0, 21],
    [12, 18, 25, 0],
];

fn route_cost(solution: &Vec<usize>) -> f64 {
    let starting_node = 0;
    let mut total = 0.0;
    let mut prev = starting_node;

    for &city in solution.iter() {
        total += DISTANCES[prev][city] as f64;
        prev = city;
    }
    total += DISTANCES[prev][starting_node] as f64;

    total
}

fn main() {
    // Define the objective function
    let objective = Objective::new(route_cost, ObjectiveType::Min);

    // Initial feasible solution (visit all cities 1...N-1)
    let initial_state: Vec<usize> = (1..NUM_CITIES).collect();

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
