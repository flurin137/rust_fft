#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use defmt::info;
use embassy_executor::Spawner;
use embedded_alloc::Heap;
use num::complex::{Complex, ComplexFloat};
use {defmt_rtt as _, panic_probe as _};

const PI: f32 = 3.1415926536;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let _peripherals = embassy_rp::init(Default::default());

    let input = vec![0.0, -0.54, 1.2, 1.8, 0.19, 0.35, 1.4, 0.0];

    let values = calculate_fft(input);

    for v in values {
        info!("Value {}", v);
    }
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

    result
}
