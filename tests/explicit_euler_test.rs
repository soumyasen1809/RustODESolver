use numerical_methods_lib::{explicit_euler_method, ode_solver};
use ode_solver::Solve;

#[cfg(test)]
mod tests {
    use super::*;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 0.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0

    #[test]
    fn explict_euler_initial_val() {
        let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

        let solver_params = ode_solver::OdeSolverParams::new(f, num_steps, T_INITIAL, TIME_STEP);
        let solver = ode_solver::OdeSolver::new("ODE Solver Explicit Euler", &solver_params);

        let explicit_euler_solver = explicit_euler_method::ExplicitEulerSolver {
            solver: Box::new(solver),
        };

        explicit_euler_solver.solve(&mut solution);

        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }
}
