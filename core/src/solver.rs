pub trait Solver {
    fn solve(&self, vars: &[f64]) -> f64;
}
pub struct DefaultSolver;

impl Solver for DefaultSolver {
    fn solve(&self, vars: &[f64]) -> f64 {
        vars.iter().sum()
    }
}
