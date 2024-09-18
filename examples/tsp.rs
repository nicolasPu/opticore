use opticore::metaheuristics::local_search::{LocalSearch, LocalSearchParameters};
use opticore::metaheuristics::{Objective, ObjectiveType};

mod utils;

// using a closure is the best way to pass the cost function?

const NUMBER_OF_NODES: usize = 10;
const GRID_SIZE: usize = 100;

fn generate_random_cost_matrix(n_nodes: usize, grid_size: usize) -> Vec<Vec<f64>> {
    let nodes = utils::generate_random_coordinates(n_nodes, grid_size);
    let mut distance_matrix = vec![vec![0.0; n_nodes]; grid_size];

    for i in 0..n_nodes {
        for j in i + 1..n_nodes {
            let distance = utils::euclidean_distance(nodes[i], nodes[j]);
            distance_matrix[i][j] = distance;
            distance_matrix[j][i] = distance; // Distance is symmetric
        }
    }
    distance_matrix
}

pub fn calculate_route_cost(solution: &Vec<usize>, cost: &Vec<Vec<f64>>) -> f64 {
    let mut route: Vec<usize> = solution.clone();
    // Add return to depot
    let starting_node = 0;
    route.insert(0, starting_node);
    route.push(starting_node);

    route.windows(2).map(|pair| cost[pair[0]][pair[1]]).sum()
}

fn main() {
    let cost_matrix = generate_random_cost_matrix(NUMBER_OF_NODES + 1, GRID_SIZE);
    // Feasible solution
    let initial_state: Vec<usize> = (1..NUMBER_OF_NODES + 1).collect();

    //let time_limit = Duration::new(10, 0); // 3 seconds, 0 nanoseconds
    let parameters = LocalSearchParameters::new(100, None); // how to no pass anything in the seed?

    // Define the objective using a closure that captures `cost_matrix`
    let cost_function = move |solution: &Vec<usize>| calculate_route_cost(solution, &cost_matrix);
    let objective = Objective::new(cost_function, ObjectiveType::Min);
    // Solve
    let mut solver = LocalSearch::new(initial_state, objective, parameters);
    let status = solver.solve();
    // Get solution
    println!("{}", status);
    println!("{}", solver.objective_value());
    println!("{:?}", solver.solution());
}
