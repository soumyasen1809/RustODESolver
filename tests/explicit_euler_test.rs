use numerical_methods_lib::ode_solvers::{explicit_euler_method, ode_solver};
use ode_solver::{Solve, WriteSolution};
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const T_INITIAL: i32 = 0; // t0
    const T_FINAL: i32 = 1; // tf
    const TIME_STEP: f64 = 0.1; // h
    const INITIAL_SOLUTION: f64 = 1.0; // S0

    const FILE_PATH: &str = "solver_results/ode_explicit_test.txt";

    fn test_fixture(solution: &mut Vec<f64>) {
        let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now
        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;

        let solver_params = ode_solver::OdeSolverParams {
            f,
            num_steps,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            ..Default::default()
        };
        let solver = ode_solver::OdeSolver::new("ODE Solver Explicit Euler Test", &solver_params);

        let explicit_euler_solver = explicit_euler_method::ExplicitEulerSolver {
            solver: Box::new(solver),
        };

        explicit_euler_solver.solve(solution);
    }

    #[test]
    fn explict_euler_initial_val() {
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];
        test_fixture(&mut solution);

        assert_eq!(*solution.get(0).unwrap(), INITIAL_SOLUTION);
    }

    #[test]
    fn write_solution_initial_val() {
        let f = |x: f64, _y: f64| 2.0 * x; // function: f(t,x)  // y marked as _y for now

        let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
        let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

        let solver_params = ode_solver::OdeSolverParams {
            f,
            num_steps,
            t_initial: T_INITIAL,
            time_step: TIME_STEP,
            ..Default::default()
        };
        let solver = ode_solver::OdeSolver::new("ODE Solver Explicit Euler Test", &solver_params);

        let explicit_euler_solver = explicit_euler_method::ExplicitEulerSolver {
            solver: Box::new(solver),
        };

        explicit_euler_solver.solve(&mut solution);
        let _write_result = explicit_euler_solver.write_solution(FILE_PATH, &solution);

        let line = read_to_string(FILE_PATH).unwrap();
        let first_line = line.lines().next().unwrap();

        assert_eq!(first_line.parse::<f64>().unwrap(), INITIAL_SOLUTION); // string.parse::<f64>().unwrap() converts a string to f64
    }
}
