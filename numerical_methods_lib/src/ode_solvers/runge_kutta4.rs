use crate::ode_solvers::ode_solver::{OdeSolver, Printable, Solve, SolverChoice};

/// Implements the Runge Kutta 4 Method.

pub struct RungeKuttaSolver<'a> {
    pub solver: Box<OdeSolver<'a>>,
}

impl<'a> RungeKuttaSolver<'a> {
    fn runge_kutta(&self, solution: &mut Vec<f64>) {
        for index in 1..self.solver.params.num_steps {
            let t_i: f64 =
                self.solver.params.t_initial as f64 + (index as f64 * self.solver.params.time_step);
            let y_i: f64 = *solution.get((index - 1) as usize).unwrap();

            let k1: f64 = self.solver.params.time_step * (self.solver.params.f)(t_i, y_i);
            let k2: f64 = self.solver.params.time_step
                * (self.solver.params.f)(
                    t_i + (self.solver.params.time_step / 2.0),
                    y_i + (k1 / 2.0),
                );
            let k3: f64 = self.solver.params.time_step
                * (self.solver.params.f)(
                    t_i + (self.solver.params.time_step / 2.0),
                    y_i + (k2 / 2.0),
                );
            let k4: f64 = self.solver.params.time_step
                * (self.solver.params.f)(t_i + self.solver.params.time_step, y_i + k3);

            let sol: f64 =
                solution.get((index - 1) as usize).unwrap() + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            solution.push(sol);
        }
    }
}

impl<'a> Solve for RungeKuttaSolver<'a> {
    /// Solves the ODE with the Runge Kutta 4 solver.
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("\n Starting RK4 Method ...");
        self.runge_kutta(solution);
    }
}

impl<'a> Printable for RungeKuttaSolver<'a> {
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

impl<'a> SolverChoice<'a> for RungeKuttaSolver<'a> {
    fn choose_solver(self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(RungeKuttaSolver {
            solver: self.solver,
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}
