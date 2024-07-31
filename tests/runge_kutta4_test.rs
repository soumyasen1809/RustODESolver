use numerical_methods_lib::runge_kutta4::*;

#[cfg(test)]
mod tests {
    use super::*;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 0.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0

    #[test]
    fn rk4_initial_value() {
        let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

        runge_kutta_4(f, num_steps, T_INITIAL, TIME_STEP, &mut solution);
        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }
}
