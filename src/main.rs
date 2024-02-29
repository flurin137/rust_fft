use num::complex::{Complex, ComplexFloat};
use std::f32::consts::PI;

fn main() {
    println!("Hello, world!");
}

fn calculate_fft(input_signal: [f32; 32]) -> [f32; 32] {
    let n = input_signal.len();

    let complex_input = input_signal.map(|a| Complex::new(a, 0.0));

    let complex = fft(n, complex_input);

    return complex.map(|x| x.abs());
}

fn fft<const N: usize, const M: usize>(n: usize, signal: [Complex<f32>; N]) -> [Complex<f32>; N]
{
    if n == 1 {
        return signal;
    }

    let n_2 = N / 2;

    let mut g_in = [Complex::new(0.0, 0.0); M];
    let mut u_in = [Complex::new(0.0, 0.0); M];

    for (index, value) in signal.iter().enumerate() {
        if index % 2 == 0 {
            g_in[index / 2] = *value;
        } else {
            u_in[index / 2] = *value;
        }
    }

    let g = fft(n_2, g_in);
    let u = fft(n_2, u_in);

    let mut result = [Complex::new(0.0, 0.0); N];

    for k in 0..(n_2) {
        let base = -2.0 * PI * Complex::i() * (k as f32) / (n as f32);
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
        let input = [
            0.0, 0.95, 1.1, 0.4, -0.4, -0.45, 0.37, 1.4, 1.7, 1.1, 0.2, -0.091, 0.54, 1.5, 2.0,
            1.5, 0.51, -0.01, 0.4, 1.3, 1.9, 1.5, 0.49, -0.24, -0.073, 0.77, 1.4, 1.2, 0.2, -0.7,
            -0.75, 0.0,
        ];

        let expected_result = [
            20.0, 7.2, 1.5, 1.0, 1.9, 15.0, 3.2, 1.6, 1.1, 0.91, 0.78, 0.69, 0.63, 0.6, 0.57, 0.56,
            0.56, 0.56, 0.57, 0.6, 0.63, 0.69, 0.78, 0.91, 1.1, 1.6, 3.2, 15.0, 1.9, 1.0, 1.5, 7.2,
        ];
        let fft = calculate_fft(input);
        print!("{fft:?}");
        assert_eq!(fft, expected_result);
    }
}
