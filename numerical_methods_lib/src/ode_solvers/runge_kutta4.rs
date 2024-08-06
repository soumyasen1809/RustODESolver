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

impl<'a> PlotSolution for RungeKuttaSolver<'a> {
    fn plot_solution(&self, solution: &Vec<f64>) {
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

impl<'a> SolverChoice<'a> for RungeKuttaSolver<'a> {
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(RungeKuttaSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a> WriteSolution<'a> for RungeKuttaSolver<'a> {
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
