use crate::solver::Solver;
//use crate::variable::{Var, VarType};

pub struct Model<S: Solver> {
    solver: S,
}

impl<S: Solver> Model<S> {
    pub fn new(solver: S) -> Self {
        Self { solver }
    }

    // pub fn add_constraint(&self, vars: &[f64]) -> None {
    //
    // }
}

// impl Default for Model<DefaultSolver> {
//     fn default() -> Self {
//         Self {
//             solver: DefaultSolver,
//         }
//     }
// }
