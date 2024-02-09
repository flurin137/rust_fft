use itertools::Itertools;
use num::complex::{Complex, ComplexFloat};
use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

fn calculate_fft(input_signal: Vec<f64>) -> Vec<f64> {
    let n = input_signal.len() / 2;

    let complex_input = input_signal
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect_vec();

    let complex = fft(n, complex_input);

    return complex.iter().map(|x| x.re).collect_vec()[..n / 2].to_vec();
}

fn fft(n: usize, signal: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    if n == 1 {
        return signal;
    }

    let mut g_in = vec![];
    let mut u_in = vec![];

    for (index, value) in signal.iter().enumerate() {
        if index % 2 == 0 {
            g_in.push(*value);
        } else {
            u_in.push(*value);
        }
    }

    let n_2 = n / 2;

    let g = fft(n_2, g_in);
    let u = fft(n_2, u_in);

    let mut x = vec![];
    println!("N {n} | G {g:?} | U {u:?}");

    for i in 0..n {
        if i % 2 == 0 {
            x.push(g[i])
        } else {
            x.push(u[i - 1])
        }
    }

    let mut result = vec![Complex::new(0.0, 0.0); n];

    for k in 0..(n_2) {
        let base = -2.0 * PI * Complex::i() * (k as f64) / (n as f64);
        let factor = base.exp();

        let p = x[k];
        let q = factor * x[k + n_2];

        result[k] = p + q;
        result[k + n_2] = p - q;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = vec![
            0.00, 0.77, 1.33, 1.54, 1.38, 0.95, 0.45, 0.09, 0.01, 0.24, 0.69, 1.18, 1.50, 1.49,
            1.09, 0.41, -0.39, -1.09, -1.48, -1.50, -1.19, -0.70, -0.25, -0.01, -0.09, -0.45,
            -0.94, -1.37, -1.54, -1.34, -0.78, -0.01,
        ];
        let expected_result = vec![];

        assert_eq!(calculate_fft(input), expected_result);
    }
}
