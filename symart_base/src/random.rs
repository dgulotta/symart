use na::Vector2;
use rand::distributions::Distribution;
use rand_distr::{Exp1, StandardNormal, Uniform};
use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};
use std::f64::consts::FRAC_PI_2;

use crate::symmetry::SymmetryGroup;

pub struct Symmetry;

impl Distribution<SymmetryGroup> for Symmetry {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SymmetryGroup {
        let idx = Uniform::new(0, SymmetryGroup::count()).sample(rng);
        SymmetryGroup::iter().skip(idx).next().unwrap()
    }
}

pub struct PointOnCircle;

impl Distribution<Vector2<f64>> for PointOnCircle {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2<f64> {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let r = (x * x + y * y).sqrt();
        Vector2::new(x / r, y / r)
    }
}

pub struct SechSquare;

impl Distribution<f64> for SechSquare {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = Uniform::new(0., 1.).sample(rng);
        f64::ln(x / (1. - x))
    }
}

pub struct Color;

impl Distribution<[u8; 3]> for Color {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [u8; 3] {
        let c = Uniform::new(0, 255).sample(rng);
        match Uniform::new(0, 6).sample(rng) {
            0 => [255, c, 0],
            1 => [255 - c, 255, 0],
            2 => [0, 255, c],
            3 => [0, 255 - c, 255],
            4 => [c, 0, 255],
            5 => [255, 0, 255 - c],
            _ => unreachable!(),
        }
    }
}

pub struct Levy {
    pub alpha: f64
}

impl Distribution<f64> for Levy {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let u = Uniform::new(-FRAC_PI_2, FRAC_PI_2).sample(rng);
        let v: f64 = Exp1.sample(rng);
        let t = (self.alpha * u).sin() / u.cos().powf(1. / self.alpha);
        let s = (((1. - self.alpha) * u).cos() / v).powf((1. - self.alpha) / self.alpha);
        t * s
    }
}
