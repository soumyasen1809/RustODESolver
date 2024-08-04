use numerical_methods_lib::*;
use ode_solvers::ode_solver::{Printable, Solve};

const T_INITIAL: i32 = 0; // t0
const T_FINAL: i32 = 1; // tf
const TIME_STEP: f64 = 0.01; // h
const INITIAL_SOLUTION: f64 = 1.0; // S0
const TOLERANCE: f64 = 1e-8; // tol
const MAX_ITERATIONS: i32 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_steps: i32 = ((T_FINAL - T_INITIAL) as f64 / TIME_STEP) as i32;
    let mut solution: Vec<f64> = vec![INITIAL_SOLUTION];

    let f = |_x: f64, _y: f64| -20.0 * _x * _y * _y; // function: f(t,x)  // y marked as _y for now
    let f_dash = |_x: f64, _y: f64| -40.0 * _x * _y; // function: f'(t,x)  // y marked as _y for now

    let solver_params = ode_solvers::ode_solver::OdeSolverParams {
        f,
        num_steps,
        t_initial: T_INITIAL,
        time_step: TIME_STEP,
        ..Default::default() // We do not need the rest for the explicit method, so we leave as Default
    };
    let solver_object =
        ode_solvers::ode_solver::OdeSolver::new("ODE Solver Explicit Euler", &solver_params);
    println!("{}", format!("{}", solver_object));

    let explicit_euler_solver = ode_solvers::explicit_euler_method::ExplicitEulerSolver {
        solver: Box::new(solver_object),
    };
    explicit_euler_solver.solve(&mut solution);
    explicit_euler_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    let solver_object = ode_solvers::ode_solver::OdeSolver::new("ODE Solver Heun", &solver_params);
    println!("{}", format!("{}", solver_object));

    let heun_method_solver = ode_solvers::heun_method::HeunSolver {
        solver: Box::new(solver_object),
    };
    heun_method_solver.solve(&mut solution);
    heun_method_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    let solver_object =
        ode_solvers::ode_solver::OdeSolver::new("ODE Solver Runge Kutta 4", &solver_params);
    println!("{}", format!("{}", solver_object));

    let rungekutta_solver = ode_solvers::runge_kutta4::RungeKuttaSolver {
        solver: Box::new(solver_object),
    };
    rungekutta_solver.solve(&mut solution);
    rungekutta_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    let solver_params_implicit = ode_solvers::ode_solver::OdeSolverParams {
        f,
        f_dash,
        num_steps,
        tolerance: TOLERANCE,
        max_iters: MAX_ITERATIONS,
        ..Default::default() // We can fill all the values, but for this case I have just filled the ones extra for the implicit
    };
    let solver_object = ode_solvers::ode_solver::OdeSolver::new(
        "ODE Solver Implicit Euler",
        &solver_params_implicit,
    );
    println!("{}", format!("{}", solver_object));

    let implicit_euler_solver = ode_solvers::implicit_euler_method::ImplicitEulerSolver {
        solver: Box::new(solver_object),
    };
    implicit_euler_solver.solve(&mut solution);
    implicit_euler_solver.print_val(&solution);
    solution.clear();
    solution.push(INITIAL_SOLUTION);

    Ok(())
}
