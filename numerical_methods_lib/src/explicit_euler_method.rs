use crate::ode_solver::{OdeSolver, Solve};

/// Implements the Euler Method.

pub struct ExplicitEulerSolver<'a> {
    pub solver: Box<OdeSolver<'a>>,
}

impl<'a> ExplicitEulerSolver<'a> {
    fn solve_euler_method(&self, solution: &mut Vec<f64>) {
        for index in 1..self.solver.params.num_steps {
            let t_i: f64 =
                self.solver.params.t_initial as f64 + (index as f64 * self.solver.params.time_step);
            let y_i: f64 = *solution.get((index - 1) as usize).unwrap();
            let sol: f64 = solution.get((index - 1) as usize).unwrap()
                + self.solver.params.time_step * (self.solver.params.f)(t_i, y_i);
            // to call the function stored in `f`, surround the field access with parentheses
            solution.push(sol);
        }
    }
}

impl<'a> Solve for ExplicitEulerSolver<'a> {
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("Starting Explicit Euler Method ...\n");
        self.solve_euler_method(solution);
    }
}
