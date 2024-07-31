/// Implements the Euler Method.
pub fn euler_method(
    f: fn(f64, f64) -> f64,
    num_steps: i32,
    t_initial: i32,
    time_step: f64,
    solution: &mut Vec<f64>,
) {
    for index in 1..num_steps {
        let t_i: f64 = t_initial as f64 + (index as f64 * time_step);
        let y_i: f64 = *solution.get((index - 1) as usize).unwrap();
        let sol: f64 = solution.get((index - 1) as usize).unwrap() + time_step * f(t_i, y_i);
        solution.push(sol);
    }
}
