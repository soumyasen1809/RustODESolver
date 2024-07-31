use numerical_methods_lib::explicit_euler_method::*;
use numerical_methods_lib::heun_method::*;

const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 0.1; // h
const INITIAL_SOLUTION: f64 = 1.0; // S0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
    let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

    let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

    euler_method(f, num_steps, T_INITIAL, TIME_STEP, &mut solution);
    println!("Solution for explicit Euler: {:?}\n", solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    heun_method(f, num_steps, T_INITIAL, TIME_STEP, &mut solution);
    println!("Solution for Heun: {:?}\n", solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    Ok(())
}
