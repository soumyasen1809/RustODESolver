use implicit_euler_method::implicit_euler_solve;
use numerical_methods_lib::*;
use ode_solver::{Printable, Solve};

const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 00.1; // h
const INITIAL_SOLUTION: f64 = 1.0; // S0
const TOLERANCE: f64 = 1e-8; // tol
const MAX_ITERATIONS: i32 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
    let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

    let f = |x: f64, _y: f64| -20.0 * x * _y * _y; // function: f(t,x)  // y marked as _y for now
    let f_dash = |_x: f64, _y: f64| -40.0 * _x * _y; // function: f'(t,x)  // y marked as _y for now

    let solver_params = ode_solver::OdeSolverParams::new(f, num_steps, T_INITIAL, TIME_STEP);
    let solver_object = ode_solver::OdeSolver::new("ODE Solver Explicit Euler", &solver_params);

    let explicit_euler_solver = explicit_euler_method::ExplicitEulerSolver {
        solver: Box::new(solver_object),
    };
    explicit_euler_solver.solve(&mut solution);
    explicit_euler_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    let solver_object2 = ode_solver::OdeSolver::new("ODE Solver Heun", &solver_params);

    let heun_method_solver = heun_method::HeunSolver {
        solver: Box::new(solver_object2),
    };
    heun_method_solver.solve(&mut solution);
    heun_method_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    let solver3 = ode_solver::OdeSolver::new("ODE Solver Runge Kutta 4", &solver_params);

    let rungekutta_solver = runge_kutta4::RungeKuttaSolver {
        solver: Box::new(solver3),
    };
    rungekutta_solver.solve(&mut solution);
    rungekutta_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

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
    println!("Solution for implicit Euler: {:.3?}\n", solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    Ok(())
}
