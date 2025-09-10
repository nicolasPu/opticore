use opticore_core::Model;
use opticore_algorithms::LocalSearch;
fn main() {
    const NUM_WORKERS: usize = 3;
    const NUM_TASKS: usize = 4;

    let costs: [[i32; NUM_TASKS]; NUM_WORKERS] =
        [[12, 18, 25, 30], [40, 22, 15, 10], [33, 27, 19, 21]];

    // 1) Create a model
    let mut model = Model();

    // 2) Variables
    let vars: [[Var; NUM_TASKS]; NUM_WORKERS] =
        costs.map(|row| row.map(|_| Var::new(VarType::Integer)));

    // 3) Constraints
    // model.

}

fn cost_function() {

}