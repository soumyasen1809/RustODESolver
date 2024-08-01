use crate::ode_solver::{OdeSolver, Solve};

/// Implements the Heun Method.

pub struct HeunSolver<'a> {
    pub solver: Box<OdeSolver<'a>>,
}

impl<'a> HeunSolver<'a> {
    fn heun_method(&self, solution: &mut Vec<f64>) {
        for index in 1..self.solver.params.num_steps {
            let t_i: f64 =
                self.solver.params.t_initial as f64 + (index as f64 * self.solver.params.time_step);
            let y_i: f64 = *solution.get((index - 1) as usize).unwrap();
            let k1: f64 = self.solver.params.time_step * (self.solver.params.f)(t_i, y_i);
            let k2: f64 = self.solver.params.time_step
                * (self.solver.params.f)(t_i + self.solver.params.time_step, y_i + k1);
            let sol: f64 = solution.get((index - 1) as usize).unwrap() + 0.5 * (k1 + k2);
            solution.push(sol);
        }
    }
}

impl<'a> Solve for HeunSolver<'a> {
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("Starting Heun Method ...\n");
        self.heun_method(solution);
    }
}
