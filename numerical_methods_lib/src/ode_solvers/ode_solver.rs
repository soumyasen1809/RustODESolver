const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 0.01; // h
const TOLERANCE: f64 = 1e-8; // tol
const MAX_ITERATIONS: i32 = 100;

/// OdeSolverParams contains all the necessary parameters for solving the ODE numerically
pub struct OdeSolverParams {
    pub f: fn(f64, f64) -> f64,
    pub f_dash: fn(f64, f64) -> f64,
    pub num_steps: i32,
    pub t_initial: i32,
    pub time_step: f64,
    pub tolerance: f64,
    pub max_iters: i32,
}

impl Default for OdeSolverParams {
    fn default() -> Self {
        OdeSolverParams {
            f: |x: f64, _y: f64| x,
            f_dash: |_x: f64, _y: f64| 1.0,
            num_steps: ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            tolerance: TOLERANCE,
            max_iters: MAX_ITERATIONS,
        }
    }
}

pub struct OdeSolver<'a> {
    pub name: &'a str,
    pub params: &'a OdeSolverParams,
}

impl<'a> OdeSolver<'a> {
    pub fn new(name: &'a str, params: &'a OdeSolverParams) -> Self {
        OdeSolver { name, params }
    }
}

pub trait Solve {
    /// Solves the ODE with the intended solver.
    fn solve(&self, solution: &mut Vec<f64>);
}

impl<'a> Solve for OdeSolver<'a> {
    fn solve(&self, solution: &mut Vec<f64>) {
        println!("\n Inside OdeSolver... with solution: {:?}", solution);
    }
}

pub trait Printable {
    fn print_val(&self, solution: &Vec<f64>);
}

impl<'a> Printable for OdeSolver<'a> {
    fn print_val(&self, solution: &Vec<f64>) {
        for (index, value) in solution.iter().enumerate() {
            println!(
                "time: {:.3} \t value: {:.3}",
                (self.params.t_initial as f64) + (index as f64 * self.params.time_step),
                *value,
            )
        }
    }
}