use numerical_methods_lib::{ode_solver, ode_solver::Solve, runge_kutta4};

#[cfg(test)]
mod tests {

    use super::*;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 0.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0

    #[test]
    fn rk4_initial_value() {
        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

        let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

        let solver_params = ode_solver::OdeSolverParams {
            f,
            num_steps,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            ..Default::default()
        };
        let solver3 = ode_solver::OdeSolver::new("ODE Solver Runge Kutta 4 Test", &solver_params);

        let rungekutta_solver = runge_kutta4::RungeKuttaSolver {
            solver: Box::new(solver3),
        };
        rungekutta_solver.solve(&mut solution);
        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }
}
