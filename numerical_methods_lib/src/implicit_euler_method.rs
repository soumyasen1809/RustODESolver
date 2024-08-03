use crate::root_finders::newton_raphson_method::*;

pub fn implicit_euler_solve(
    f: fn(f64, f64) -> f64,
    f_dash: fn(f64, f64) -> f64,
    num_steps: i32,
    t_initial: i32,
    time_step: f64,
    tol: f64,
    max_iters: i32,
    solution: &mut Vec<f64>,
) {
    println!("Using Newton Raphson method to find roots ...");
    for index in 0..(num_steps - 1) {
        let g = |z: f64| {
            z - solution.get(index as usize).unwrap()
                - time_step * f(t_initial as f64 + time_step * ((index + 1) as f64), z)
        };
        let g_dash = |z: f64| {
            1.0 - time_step * f_dash(t_initial as f64 + time_step * ((index + 1) as f64), z)
        };
        // Note: since for g and g_dash we are using f and f_dash which are outside the fn,
        // we need to use the Fn trait in the newton_method.rs file
        // Else, we get the error: closures can only be coerced to `fn` types if
        // they do not capture any variables rustc E0308.
        // Check solution at:
        // https://www.reddit.com/r/learnrust/comments/xvxpy2/is_there_a_workaround_for_variable_capturing_in/

        let newton_sol = newton_raphson_method_root(
            g,
            g_dash,
            *solution.get(index as usize).unwrap(),
            tol,
            max_iters,
        );

        solution.push(newton_sol);
    }
}
