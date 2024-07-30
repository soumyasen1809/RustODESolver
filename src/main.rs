const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 0.1; // h
const INITIAL_SOLUTION: f64 = 1.0; // S0

/// Implements the Euler Method.
fn euler_method(f: fn(f64, f64) -> f64, num_steps: i32, solution: &mut Vec<f64>) {
    for index in 1..num_steps {
        let t_i: f64 = T_INITIAL as f64 + (index as f64 * TIME_STEP);
        let y_i: f64 = *solution.get((index - 1) as usize).unwrap();
        let sol: f64 = solution.get((index - 1) as usize).unwrap() + TIME_STEP * f(t_i, y_i);
        solution.push(sol);
    }
}

/// Implements the Heun Method.
fn heun_method(f: fn(f64, f64) -> f64, num_steps: i32, solution: &mut Vec<f64>) {
    for index in 1..num_steps {
        let t_i: f64 = T_INITIAL as f64 + (index as f64 * TIME_STEP);
        let y_i: f64 = *solution.get((index - 1) as usize).unwrap();
        let k1: f64 = TIME_STEP * f(t_i, y_i);
        let k2: f64 = TIME_STEP * f(t_i + TIME_STEP, y_i + k1);
        let sol: f64 = solution.get((index - 1) as usize).unwrap() + 0.5 * (k1 + k2);
        solution.push(sol);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
    let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

    let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

    euler_method(f, num_steps, &mut solution);
    println!("Solution for explicit Euler: {:?}\n", solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    heun_method(f, num_steps, &mut solution);
    println!("Solution for Heun: {:?}\n", solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    Ok(())
}
