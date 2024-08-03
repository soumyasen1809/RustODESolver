use numerical_methods_lib::{implicit_euler_method, ode_solver, ode_solver::Solve};

#[cfg(test)]
mod tests {
    use super::*;

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

        let solver_params = ode_solver::OdeSolverParams {
            f,
            f_dash,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            num_steps: num_steps,
            tolerance: TOLERANCE,
            max_iters: MAX_ITERATIONS,
            ..Default::default()
        };
        let ode_solver = ode_solver::OdeSolver::new("Implicit Euler Method Test", &solver_params);
        let implicit_solver = implicit_euler_method::ImplicitEulerSolver {
            solver: Box::new(ode_solver),
        };
        implicit_solver.solve(&mut solution);
        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }
}