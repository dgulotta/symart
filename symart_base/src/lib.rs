extern crate image;
extern crate nalgebra as na;
extern crate ndarray;
extern crate num_complex;
extern crate num_traits;
extern crate ordered_float;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand_distr;
#[cfg(feature = "threads")]
extern crate rayon;
extern crate rustfft;
extern crate serde_json;
extern crate strum;
extern crate strum_macros;

pub mod canvas;
pub mod fft;
pub mod layer;
pub mod random;
pub mod rng;
pub mod schema;
pub mod symmetric_canvas;
pub mod symmetry;

use image::RgbImage;
#[cfg(feature = "threads")]
use rayon::prelude::*;

use crate::rng::sample;
use crate::symmetry::SymmetryGroup;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SymmetryChoice {
    Random,
    #[serde(untagged)]
    Symmetry(SymmetryGroup),
}

impl From<SymmetryChoice> for SymmetryGroup {
    fn from(sc: SymmetryChoice) -> Self {
        match sc {
            SymmetryChoice::Symmetry(s) => s,
            SymmetryChoice::Random => sample(random::Symmetry),
        }
    }
}

pub trait Design: serde::de::DeserializeOwned {
    fn name() -> &'static str;
    fn schema() -> serde_json::Value;
    fn draw(&self) -> Result<DrawResponse, Box<dyn std::error::Error>>;
}

pub enum SymmetryType {
    Wrapped(SymmetryGroup),
    None,
}

impl From<SymmetryGroup> for SymmetryType {
    fn from(g: SymmetryGroup) -> SymmetryType {
        SymmetryType::Wrapped(g)
    }
}

pub struct DrawResponse {
    pub im: RgbImage,
    pub sym: SymmetryType,
}

#[cfg(feature = "threads")]
pub fn make_layers_n<F, T: Send>(n: usize, f: F) -> impl Iterator<Item = T>
where
    F: Fn(usize) -> T + Send + Sync,
{
    (0..n)
        .into_par_iter()
        .map(f)
        .collect::<Vec<_>>()
        .into_iter()
}

#[cfg(not(feature = "threads"))]
pub fn make_layers_n<F, T: Send>(n: usize, f: F) -> impl Iterator<Item = T>
where
    F: Fn(usize) -> T + Send + Sync,
{
    (0..n).map(f)
}

pub fn make_layers<F, T: Send>(n: usize, f: F) -> impl Iterator<Item = T>
where
    F: Fn() -> T + Send + Sync,
{
    make_layers_n(n, move |_| f())
}
