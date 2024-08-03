// Method used from:
// https://de.mathworks.com/matlabcentral/answers/1716860-using-implicit-euler-method-with-newton-raphson-method

// Why we are using the f: impl Fn(..) -> .. instead of f: fn(..) -> ..
// Check https://www.reddit.com/r/learnrust/comments/xvxpy2/is_there_a_workaround_for_variable_capturing_in/
pub fn newton_raphson_method_root(
    f: impl Fn(f64) -> f64,
    f_dash: impl Fn(f64) -> f64,
    mut x_initial: f64,
    tol: f64,
    max_iters: i32,
) -> f64 {
    let mut num_iters = 0;
    let f_abs = f(x_initial).abs();

    while f_abs > tol {
        let x_temp = x_initial - (f(x_initial) / f_dash(x_initial));
        x_initial = x_temp;
        num_iters += 1;

        if num_iters > max_iters {
            // println!("\n Max iterations {:?} reached", num_iters);
            break;
        }
    }

    x_initial
}
