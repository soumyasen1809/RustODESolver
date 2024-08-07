// Method used from:
// https://de.mathworks.com/matlabcentral/answers/1716860-using-implicit-euler-method-with-newton-raphson-method
pub fn newton_raphson_method_root<T>(
    f: impl Fn(T) -> T,
    f_dash: impl Fn(T) -> T,
    mut x_initial: T,
    tol: f64,
    max_iters: i32,
) -> T
where
    T: std::ops::Sub<T, Output = T> + std::ops::Div<T, Output = T> + Into<f64> + Copy,
{
    let mut num_iters = 0;
    let f_abs = f(x_initial).into().abs();

    while f_abs > tol {
        let x_temp = x_initial - (f(x_initial) / f_dash(x_initial));
        x_initial = x_temp;
        num_iters += 1;

        if num_iters > max_iters {
            break;
        }
    }

    x_initial
}
