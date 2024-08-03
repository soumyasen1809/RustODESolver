#[cfg(test)]
mod tests {
    use numerical_methods_lib::implicit_euler_method::implicit_euler_solve;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 00.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0
    const TOLERANCE: f64 = 1e-8; // tol
    const MAX_ITERATIONS: i32 = 100;

    #[test]
    fn implict_euler_initial_val() {
        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

        let f = |_x: f64, _y: f64| -20.0 * _x * _y * _y; // function: f(t,x)  // y marked as _y for now
        let f_dash = |_x: f64, _y: f64| -40.0 * _x * _y; // function: f'(t,x)  // y marked as _y for now

        implicit_euler_solve(
            f,
            f_dash,
            num_steps,
            T_INITIAL,
            TIME_STEP,
            TOLERANCE,
            MAX_ITERATIONS,
            &mut solution,
        );

        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }
}
