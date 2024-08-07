const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 0.01; // h
const TOLERANCE: f64 = 1e-8; // tol
const MAX_ITERATIONS: i32 = 100;

/// OdeSolverParams contains all the necessary parameters for solving the ODE numerically
pub struct OdeSolverParams<T> {
    pub f: fn(f64, T) -> f64,
    pub f_dash: fn(f64, T) -> f64,
    pub num_steps: i32,
    pub t_initial: i32,
    pub time_step: f64,
    pub tolerance: f64,
    pub max_iters: i32,
}

impl<T> Default for OdeSolverParams<T> {
    fn default() -> Self {
        OdeSolverParams {
            f: |_x: f64, _y: T| _x,
            f_dash: |_x: f64, _y: T| 1.0,
            num_steps: ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            tolerance: TOLERANCE,
            max_iters: MAX_ITERATIONS,
        }
    }
}

#[derive(Clone, Copy)]
pub struct OdeSolver<'a, T> {
    pub name: &'a str,
    pub params: &'a OdeSolverParams<T>,
}

impl<'a, T> OdeSolver<'a, T> {
    pub fn new(name: &'a str, params: &'a OdeSolverParams<T>) -> Self {
        OdeSolver { name, params }
    }
}

pub trait Solve<T> {
    /// Solves the ODE with the intended solver.
    fn solve(&self, solution: &mut Vec<T>);
}

pub trait Printable<T> {
    fn print_val(&self, solution: &Vec<T>);
}

pub trait PlotSolution<T> {
    fn plot_solution(&self, solution: &Vec<T>);
}

pub trait SolverChoice<'a> {
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a>;
    // By adding + 'a to the return type, you're specifying that
    // the returned Box<dyn SolverChoice<'a>> must live at least as long as 'a.
    // This ensures that the returned value doesn't outlive the lifetime of self

    fn name_solver(&self) -> &'a str;
}

impl<'a, T> SolverChoice<'a> for OdeSolver<'a, T> {
    fn choose_solver(&self) -> Box<dyn SolverChoice<'a> + 'a> {
        Box::new(OdeSolver {
            name: "Ode Solver",
            params: &self.params,
        })
    }

    fn name_solver(&self) -> &'a str {
        self.name
    }
}

pub trait WriteSolution<'a, T> {
    fn write_solution(
        &self,
        file_path: &'a str,
        solution: &Vec<T>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
