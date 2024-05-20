use ndarray::Array2;
use num_complex::Complex64;
use num_traits::Zero;
use rustfft::{Fft, FftDirection, FftPlanner};
use std::sync::Arc;
use transpose::transpose_inplace;

#[derive(Clone)]
pub struct Plan2D {
    horizontal: Arc<dyn Fft<f64>>,
    vertical: Arc<dyn Fft<f64>>,
}

impl Plan2D {
    pub fn new(planner: &mut FftPlanner<f64>, width: usize, height: usize) -> Self {
        Self {
            horizontal: planner.plan_fft(width, FftDirection::Forward),
            vertical: planner.plan_fft(height, FftDirection::Forward),
        }
    }

    pub fn apply(&self, arr: &mut Array2<Complex64>) {
        let w = arr.shape()[0];
        let h = arr.shape()[1];
        let sdim = *[
            self.horizontal.get_inplace_scratch_len(),
            self.vertical.get_inplace_scratch_len(),
            w,
            h,
        ]
        .iter()
        .max()
        .unwrap();
        let mut scratch = vec![Zero::zero(); sdim];
        let flat = arr.as_slice_mut().unwrap();
        self.vertical.process_with_scratch(flat, &mut scratch);
        transpose_inplace(flat, &mut scratch, h, w);
        self.horizontal.process_with_scratch(flat, &mut scratch);
        transpose_inplace(flat, &mut scratch, w, h);
    }

    pub fn width(&self) -> usize {
        self.horizontal.len()
    }

    pub fn height(&self) -> usize {
        self.vertical.len()
    }
}
