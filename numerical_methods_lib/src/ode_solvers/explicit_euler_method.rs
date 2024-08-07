use crate::ode_solvers::ode_solver::{
    OdeSolver, PlotSolution, Printable, Solve, SolverChoice, WriteSolution,
};
use plotly::{
    common::{Marker, Mode},
    layout::Axis,
    Layout, Plot, Scatter,
};
use std::{fs::File, io::Write};

/// Implements the Euler Method.

pub struct ExplicitEulerSolver<'a, T> {
    pub solver: Box<OdeSolver<'a, T>>,
}

impl<'a, T> ExplicitEulerSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T> + Copy,
{
    fn solve_euler_method(&self, solution: &mut Vec<T>) {
        for index in 1..self.solver.params.num_steps {
            let t_i: f64 =
                self.solver.params.t_initial as f64 + (index as f64 * self.solver.params.time_step);
            let y_i: T = *solution.get((index - 1) as usize).unwrap();
            let sol: T = *solution.get((index - 1) as usize).unwrap()
                + self.solver.params.time_step * (self.solver.params.f)(t_i, y_i);
            // to call the function stored in `f`, surround the field access with parentheses
            solution.push(sol);
        }
    }
}

impl<'a, T> Solve<T> for ExplicitEulerSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T> + Copy,
{
    /// Solves the ODE with the Explicit Euler Method solver.
    fn solve(&self, solution: &mut Vec<T>) {
        println!("\n Starting Explicit Euler Method ...");
        self.solve_euler_method(solution);
    }
}

impl<'a, T> Printable<T> for ExplicitEulerSolver<'a, T>
where
    T: std::fmt::Display,
{
    fn print_val(&self, solution: &Vec<T>) {
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

impl<'a, T> PlotSolution<T> for ExplicitEulerSolver<'a, T>
where
    T: serde::ser::Serialize + Clone + 'static,
{
    fn plot_solution(&self, solution: &Vec<T>) {
        let t_array: Vec<f64> = solution
            .iter()
            .enumerate()
            .map(|(index, _)| {
                (self.solver.params.t_initial as f64)
                    + (index as f64 * self.solver.params.time_step)
            })
            .collect();

        let sol_trace = Scatter::new(t_array, solution.to_vec())
            .mode(Mode::Markers)
            .marker(Marker::new().size(1));
        let mut scatter_plot = Plot::new();
        scatter_plot.add_trace(sol_trace);
        let plot_layout = Layout::new()
            .title("Explicit Euler Plot")
            .x_axis(Axis::new().title("solution"))
            .y_axis(Axis::new().title("time"));
        scatter_plot.set_layout(plot_layout);

        scatter_plot.write_html("solver_results/images/explicit_euler.html");
    }
}

impl<'a, T> SolverChoice<'a> for ExplicitEulerSolver<'a, T>
where
    T: Copy,
{
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(ExplicitEulerSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a, T> WriteSolution<'a, T> for ExplicitEulerSolver<'a, T>
where
    T: std::fmt::Display,
{
    fn write_solution(
        &self,
        file_path: &'a str,
        solution: &Vec<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&file_path)?;

        for val in solution {
            writeln!(file, "{}", val)?;
        }

        Ok(())
    }
}
