pub struct OdeSolverParams {
    pub f: fn(f64, f64) -> f64,
    pub num_steps: i32,
    pub t_initial: i32,
    pub time_step: f64,
}

impl OdeSolverParams {
    pub fn new(f: fn(f64, f64) -> f64, num_steps: i32, t_initial: i32, time_step: f64) -> Self {
        OdeSolverParams {
            f,
            num_steps,
            t_initial,
            time_step,
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
