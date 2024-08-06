use numerical_methods_lib::ode_solvers::{ode_solver, runge_kutta4};
use ode_solver::{Solve, WriteSolution};
use std::fs::read_to_string;

#[cfg(test)]
mod tests {

    use super::*;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 0.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0

    const FILE_PATH: &str = "solver_results/rk4_solver_test.txt";

    fn test_fixture(solution: &mut Vec<f64>) {
        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
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
        rungekutta_solver.solve(solution);
    }

    #[test]
    fn rk4_initial_value() {
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];
        test_fixture(&mut solution);

        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }

    #[test]
    fn write_solution_initial_val() {
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
        let _write_result = rungekutta_solver.write_solution(FILE_PATH, &solution);

        let line = read_to_string(FILE_PATH).unwrap();
        let first_line = line.lines().next().unwrap();

        assert_eq!(first_line.parse::<f64>().unwrap(), INITIAL_SOLUTION);
    }
}
