use crate::{
    ode_solvers::ode_solver::{
        OdeSolver, PlotSolution, Printable, Solve, SolverChoice, WriteSolution,
    },
    root_finders::newton_raphson_method::*,
};
use plotly::{
    common::{Marker, Mode},
    layout::Axis,
    Layout, Plot, Scatter,
};
use std::{fs::File, io::Write};

pub struct ImplicitEulerSolver<'a, T> {
    pub solver: Box<OdeSolver<'a, T>>,
}

impl<'a, T> ImplicitEulerSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T>
        + std::ops::Sub<f64, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::convert::From<f64>
        + Into<f64>
        + std::ops::Div<Output = T>
        + Copy,
{
    fn implicit_euler_method(&self, solution: &mut Vec<T>) {
        println!("\n Using Newton Raphson method to find roots ...");
        for index in 0..(self.solver.params.num_steps - 1) {
            let g = |z: T| {
                z - *solution.get(index as usize).unwrap()
                    - self.solver.params.time_step
                        * (self.solver.params.f)(
                            self.solver.params.t_initial as f64
                                + self.solver.params.time_step * ((index + 1) as f64),
                            z,
                        )
            };
            let g_dash = |z: T| {
                (1.0 - self.solver.params.time_step
                    * (self.solver.params.f_dash)(
                        self.solver.params.t_initial as f64
                            + self.solver.params.time_step * ((index + 1) as f64),
                        z,
                    ))
                .into()
            };

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

impl<'a, T> Solve<T> for ImplicitEulerSolver<'a, T>
where
    T: std::ops::Add<f64, Output = T>
        + std::ops::Sub<f64, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::convert::From<f64>
        + Into<f64>
        + std::ops::Div<Output = T>
        + Copy,
{
    fn solve(&self, solution: &mut Vec<T>) {
        println!("\n Starting Implicit Euler Method ...");
        self.implicit_euler_method(solution);
    }
}

impl<'a, T> Printable<T> for ImplicitEulerSolver<'a, T>
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

impl<'a, T> PlotSolution<T> for ImplicitEulerSolver<'a, T>
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
            .title("Implicit Euler Plot")
            .x_axis(Axis::new().title("solution"))
            .y_axis(Axis::new().title("time"));
        scatter_plot.set_layout(plot_layout);

        scatter_plot.write_html("solver_results/images/implicit_euler.html");
    }
}

impl<'a, T> SolverChoice<'a> for ImplicitEulerSolver<'a, T>
where
    T: Copy,
{
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(ImplicitEulerSolver {
            solver: Box::new(*self.solver),
        })
    }

    fn name_solver(&self) -> &'a str {
        self.solver.name
    }
}

impl<'a, T> WriteSolution<'a, T> for ImplicitEulerSolver<'a, T>
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
