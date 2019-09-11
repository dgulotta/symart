use ndarray::Array2;
use rustfft::{FFTplanner, FFT};
use std::sync::Arc;
use transpose::transpose;
use num_complex::Complex64;
use num_traits::Zero;

#[derive(Clone)]
pub struct Plan2D {
    horizontal: Arc<dyn FFT<f64>>,
    vertical: Arc<dyn FFT<f64>>
}

impl Plan2D {
    pub fn new(planner: &mut FFTplanner<f64>, width: usize, height: usize) -> Self {
        Self {
            horizontal: planner.plan_fft(width),
            vertical: planner.plan_fft(height)
        }
    }

    pub fn apply(&self, arr: &mut Array2<Complex64>) {
        let w = arr.shape()[0];
        let h = arr.shape()[1];
        let flat = arr.as_slice_mut().unwrap();
        let mut buf = vec! [Zero::zero(); flat.len()];
        self.vertical.process_multi(flat, &mut buf);
        transpose(&buf, flat, h, w);
        self.horizontal.process_multi(flat, &mut buf);
        transpose(&buf, flat, w, h);
    }

    pub fn width(&self) -> usize { self.horizontal.len() }

    pub fn height(&self) -> usize { self.vertical.len() }
}
