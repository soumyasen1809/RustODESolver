use crate::{
    ode_solvers::ode_solver::{
        OdeSolver, PlotSolution, Printable, Solve, SolverChoice, WriteSolution,
    },
    root_finders::newton_raphson_method::*,
};
use plotly::{Plot, Scatter};
use std::{fs::File, io::Write};

pub struct ImplicitEulerSolver<'a> {
    pub solver: Box<OdeSolver<'a>>,
}

impl<'a> ImplicitEulerSolver<'a> {
    fn implicit_euler_method(&self, solution: &mut Vec<f64>) {
        println!("\n Using Newton Raphson method to find roots ...");
        for index in 0..(self.solver.params.num_steps - 1) {
            let g = |z: f64| {
                z - solution.get(index as usize).unwrap()
                    - self.solver.params.time_step
                        * (self.solver.params.f)(
                            self.solver.params.t_initial as f64
                                + self.solver.params.time_step * ((index + 1) as f64),
                            z,
                        )
            };
            let g_dash = |z: f64| {
                1.0 - self.solver.params.time_step
                    * (self.solver.params.f_dash)(
                        self.solver.params.t_initial as f64
                            + self.solver.params.time_step * ((index + 1) as f64),
                        z,
                    )
            };
            // Note: since for g and g_dash we are using f and f_dash which are outside the fn,
            // we need to use the Fn trait in the newton_method.rs file
            // Else, we get the error: closures can only be coerced to `fn` types if
            // they do not capture any variables rustc E0308.
            // Check solution at:
            // https://www.reddit.com/r/learnrust/comments/xvxpy2/is_there_a_workaround_for_variable_capturing_in/

            let newton_sol = newton_raphson_method_root(
                g,
                g_dash,
                *solution.get(index as usize).unwrap(),
                self.solver.params.tolerance,
                self.solver.params.max_iters,
            );

            solution.push(newton_sol);
        }
    }
}

impl<'a> Solve for ImplicitEulerSolver<'a> {
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("\n Starting Implicit Euler Method ...");
        self.implicit_euler_method(solution);
    }
}

impl<'a> Printable for ImplicitEulerSolver<'a> {
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

impl<'a> PlotSolution for ImplicitEulerSolver<'a> {
    fn plot_solution(&self, solution: &Vec<f64>) {
        let t_array: Vec<f64> = solution
            .iter()
            .enumerate()
            .map(|(index, _)| {
                (self.solver.params.t_initial as f64)
                    + (index as f64 * self.solver.params.time_step)
            })
            .collect();

        let sol_trace = Scatter::new(t_array, solution.to_vec());
        let mut scatter_plot = Plot::new();
        scatter_plot.add_trace(sol_trace);

        scatter_plot.write_html("solver_results/images/implicit_euler.html");
    }
}

impl<'a> SolverChoice<'a> for ImplicitEulerSolver<'a> {
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(ImplicitEulerSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a> WriteSolution<'a> for ImplicitEulerSolver<'a> {
    fn write_solution(
        &self,
        file_path: &'a str,
        solution: &Vec<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&file_path)?;

        for val in solution {
            writeln!(file, "{}", val)?;
        }

        Ok(())
    }
}
