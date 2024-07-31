/// Implements the Runge Kutta 4 Method.
pub fn runge_kutta_4(
    f: fn(f64, f64) -> f64,
    num_steps: i32,
    t_initial: i32,
    time_step: f64,
    solution: &mut Vec<f64>,
) {
    for index in 1..num_steps {
        let t_i: f64 = t_initial as f64 + (index as f64 * time_step);
        let y_i: f64 = *solution.get((index - 1) as usize).unwrap();

        let k1: f64 = time_step * f(t_i, y_i);
        let k2: f64 = time_step * f(t_i + (time_step / 2.0), y_i + (k1 / 2.0));
        let k3: f64 = time_step * f(t_i + (time_step / 2.0), y_i + (k2 / 2.0));
        let k4: f64 = time_step * f(t_i + time_step, y_i + k3);

        let sol: f64 =
            solution.get((index - 1) as usize).unwrap() + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        solution.push(sol);
    }
}
