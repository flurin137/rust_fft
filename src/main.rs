use itertools::Itertools;
use num::complex::{Complex, ComplexFloat};
use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

fn calculate_fft(input_signal: Vec<f64>) -> Vec<f64> {
    let n = input_signal.len();

    let complex_input = input_signal
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect_vec();

    let complex = fft(n, complex_input);

    return complex.iter().map(|x| x.abs()).collect_vec().to_vec();
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

    let mut result = vec![Complex::new(0.0, 0.0); n];

    for k in 0..(n_2) {
        let base = -2.0 * PI * Complex::i() * (k as f64) / (n as f64);
        let factor = base.exp();

        result[k] = g[k] + u[k] * factor;
        result[k + n_2] = g[k] - u[k] * factor;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = vec![
            0.0, 0.95, 1.1, 0.4, -0.4, -0.45, 0.37, 1.4, 1.7, 1.1, 0.2, -0.091, 0.54, 1.5, 2.0,
            1.5, 0.51, -0.01, 0.4, 1.3, 1.9, 1.5, 0.49, -0.24, -0.073, 0.77, 1.4, 1.2, 0.2, -0.7,
            -0.75, 0.0,
        ];

        let expected_result = vec![
            20.0, 7.2, 1.5, 1.0, 1.9, 15.0, 3.2, 1.6, 1.1, 0.91, 0.78, 0.69, 0.63, 0.6, 0.57, 0.56,
            0.56, 0.56, 0.57, 0.6, 0.63, 0.69, 0.78, 0.91, 1.1, 1.6, 3.2, 15.0, 1.9, 1.0, 1.5, 7.2,
        ];
        let fft = calculate_fft(input);
        print!("{fft:?}");
        assert_eq!(fft, expected_result);
    }

    #[test]
    fn _1() {
        let input = vec![0.0, -0.54, 1.2, 1.8, 0.19, 0.35, 1.4, 0.0];

        let expected_result = vec![
            4.38128627, 2.10632708, 3.11669564, 1.8615992, 1.25396034, 1.8615992, 3.11669564,
            2.10632708,
        ];
        let fft = calculate_fft(input);
        print!("{fft:?}");
        assert_eq!(fft, expected_result);
    }
}
