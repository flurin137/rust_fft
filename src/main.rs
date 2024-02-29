#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use num::complex::{Complex, ComplexFloat};

const PI: f32 = 3.1415926536;

#[panic_handler]
fn asdf(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    loop {}
}

fn calculate_fft(input_signal: Vec<f32>) -> Vec<f32> {
    let n = input_signal.len();

    let complex_input = input_signal.iter().map(|x| Complex::new(*x, 0.0)).collect();

    let complex = fft(n, complex_input);

    complex.iter().map(|x| x.abs()).collect()
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
        let factor = base.exp();

        result[k] = g[k] + u[k] * factor;
        result[k + n_2] = g[k] - u[k] * factor;
    }

    return result;
}
