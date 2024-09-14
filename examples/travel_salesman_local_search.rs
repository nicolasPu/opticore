use std::time::Duration;
use opticore::metaheuristics::local_search::{LocalSearch, LocalSearchParameters};
use opticore::metaheuristics::{ObjectiveType, Solution};

pub fn route_cost(solution: &Vec<usize>) -> f64 {
    let cost = vec![
        vec![0., 10., 15., 20., 25.,],
        vec![10., 0., 35., 25., 30.,],
        vec![15., 35., 0., 30., 20.,],
        vec![20., 25., 30., 0., 15.,],
        vec![25., 30., 20., 15., 0.,],
    ];

    solution.iter().map(|&node| cost[0][node]).sum()
}

fn main() {
    let initial_state: Vec<usize> = (0..5).collect();
    let time_limit = Duration::new(10,0); // 3 seconds, 0 nanoseconds
    let parameters = LocalSearchParameters::new(100, time_limit, None); // how to no pass anything in the seed?
    let solver = LocalSearch::new(route_cost, ObjectiveType::Min, initial_state, parameters);
    let solution: Solution = solver.solve();

    solution.report()
}