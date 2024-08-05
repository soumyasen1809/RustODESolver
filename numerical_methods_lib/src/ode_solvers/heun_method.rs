use crate::ode_solvers::ode_solver::{OdeSolver, Printable, Solve, SolverChoice};

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
    /// Solves the ODE with the Heun solver.
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("\n Starting Heun Method ...");
        self.heun_method(solution);
    }
}

impl<'a> Printable for HeunSolver<'a> {
    fn print_val(&self, solution: &Vec<f64>) {
        for (index, value) in solution.iter().enumerate() {
            println!(
                "time: {:.3} \t value: {:.3}",
                (self.solver.params.t_initial as f64)
                    + (index as f64 * self.solver.params.time_step),
                *value,
            )
        }
    }
}

impl<'a> SolverChoice<'a> for HeunSolver<'a> {
    fn choose_solver(self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(HeunSolver {
            solver: self.solver,
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}
