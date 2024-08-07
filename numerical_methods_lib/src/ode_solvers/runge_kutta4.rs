use crate::ode_solvers::ode_solver::{
    OdeSolver, PlotSolution, Printable, Solve, SolverChoice, WriteSolution,
};
use plotly::{
    common::{Marker, Mode},
    layout::Axis,
    Layout, Plot, Scatter,
};
use std::{fs::File, io::Write};

/// Implements the Runge Kutta 4 Method.

pub struct RungeKuttaSolver<'a, T> {
    pub solver: Box<OdeSolver<'a, T>>,
}

impl<'a, T> RungeKuttaSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T> + Copy,
{
    fn runge_kutta(&self, solution: &mut Vec<T>) {
        for index in 1..self.solver.params.num_steps {
            let t_i: f64 =
                self.solver.params.t_initial as f64 + (index as f64 * self.solver.params.time_step);
            let y_i: T = *solution.get((index - 1) as usize).unwrap();

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

            let sol: T = *solution.get((index - 1) as usize).unwrap()
                + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            solution.push(sol);
        }
    }
}

impl<'a, T> Solve<T> for RungeKuttaSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T> + Copy,
{
    /// Solves the ODE with the Runge Kutta 4 solver.
    fn solve(&self, solution: &mut Vec<T>) {
        println!("\n Starting RK4 Method ...");
        self.runge_kutta(solution);
    }
}

impl<'a, T> Printable<T> for RungeKuttaSolver<'a, T>
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

impl<'a, T> PlotSolution<T> for RungeKuttaSolver<'a, T>
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
            .title("RK4 Plot")
            .x_axis(Axis::new().title("solution"))
            .y_axis(Axis::new().title("time"));
        scatter_plot.set_layout(plot_layout);

        scatter_plot.write_html("solver_results/images/rungekutta.html");
    }
}

impl<'a, T> SolverChoice<'a> for RungeKuttaSolver<'a, T>
where
    T: Copy,
{
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(RungeKuttaSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a, T> WriteSolution<'a, T> for RungeKuttaSolver<'a, T>
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
