use crate::ode_solvers::ode_solver::{
    OdeSolver, PlotSolution, Printable, Solve, SolverChoice, WriteSolution,
};
use plotly::{Plot, Scatter};
use std::{fs::File, io::Write};

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
    /// Solves the ODE with the Explicit Euler Method solver.
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("\n Starting Explicit Euler Method ...");
        self.solve_euler_method(solution);
    }
}

impl<'a> Printable for ExplicitEulerSolver<'a> {
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

impl<'a> PlotSolution for ExplicitEulerSolver<'a> {
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

        scatter_plot.write_html("solver_results/images/explicit_euler.html");
    }
}

impl<'a> SolverChoice<'a> for ExplicitEulerSolver<'a> {
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(ExplicitEulerSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a> WriteSolution<'a> for ExplicitEulerSolver<'a> {
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
