extern crate alga;
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
#[cfg(feature = "threads")]
extern crate rayon;
extern crate rand_distr;
extern crate rustfft;
extern crate serde_json;
extern crate strum;
extern crate strum_macros;

pub mod canvas;
pub mod color;
pub mod layer;
pub mod random;
pub mod rng;
pub mod schema;
pub mod symmetric_canvas;
pub mod symmetry;

use image::RgbImage;
#[cfg(feature = "threads")]
use rayon::prelude::*;

use crate::symmetry::SymmetryGroup;
use crate::rng::sample;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RandomSymmetry {
    Random,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SymmetryChoice {
    Symmetry(SymmetryGroup),
    Random(RandomSymmetry),
}

impl From<SymmetryChoice> for SymmetryGroup {
    fn from(sc: SymmetryChoice) -> Self {
        match sc {
            SymmetryChoice::Symmetry(s) => s,
            SymmetryChoice::Random(_) => sample(random::Symmetry)
        }
    }
}

pub trait Design: serde::de::DeserializeOwned {
    fn schema() -> serde_json::Value;
    fn draw(&self) -> DrawResponse;
}

pub struct DrawResponse {
    pub im: RgbImage,
    pub sym: SymmetryGroup,
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
    F: Fn() -> T + Send + Sync
{
    make_layers_n(n, move |_| f())
}
