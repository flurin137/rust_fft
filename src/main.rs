use num::complex::Complex;

const PI: f32 = 3.1415926536;
const E: f32 = 2.7182818285;

fn main() {
    println!("Hello, world!");
}

fn calculate_fft(input_signal: Vec<f32>) -> Vec<f32> {
    let n = input_signal.len();

    let complex_input = input_signal.iter().map(|x| Complex::new(*x, 0.0)).collect();

    let complex = fft(n, complex_input);

    complex.iter().map(|x| abs(*x)).collect()
}

fn abs(number: Complex<f32>) -> f32 {
    f32::sqrt(number.im * number.im + number.re * number.re)
}

fn exp(number: Complex<f32>) -> Complex<f32> {
    let r = number.re.exp();
    let theta = number.im;
    Complex::new(r * theta.cos(), r * theta.sin())
}

fn fft(n: usize, signal: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
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
        let base = -2.0 * PI * Complex::i() * (k as f32) / (n as f32);
        let factor = exp(base);

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
            19.716, 7.1356616, 1.5019984, 0.9707915, 1.8354987, 15.065951, 3.1998298, 1.5920978,
            1.1611935, 0.88543063, 0.87258303, 0.62757486, 0.7070812, 0.5255001, 0.51221985,
            0.6415046, 0.5419998, 0.6415048, 0.5122198, 0.52550036, 0.7070813, 0.6275738,
            0.87258315, 0.8854304, 1.1611935, 1.5920982, 3.1998298, 15.065952, 1.8354987,
            0.9707922, 1.5019984, 7.135661,
        ];
        let fft = calculate_fft(input);
        print!("{fft:?}");
        assert_eq!(fft, expected_result);
    }

    #[test]
    fn _1() {
        let input = vec![0.0, -0.54, 1.2, 1.8, 0.19, 0.35, 1.4, 0.0];

        let expected_result = vec![
            4.3999996, 2.1386015, 3.1254117, 1.9086071, 1.1800001, 1.9086074, 3.1254117, 2.1386015,
        ];
        let fft = calculate_fft(input);
        print!("{fft:?}");
        assert_eq!(fft, expected_result);
    }
}
