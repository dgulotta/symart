use image::RgbImage;
use ndarray::{Array2, indices_of};
use num_complex::Complex64;
use rand::Rng;
use rand::distributions::Distribution;
use rustfft::FFTplanner;
use std::f64::consts::PI;
use symart_base::canvas::{Coord, WrapCanvas, WrapDimension};
use symart_base::fft::Plan2D;
use symart_base::random::Levy;
use symart_base::rng::sample_fn;
use symart_base::symmetry::{SymmetryGroup, transformations};
use symart_base::symmetric_canvas::SymmetricCanvas;
use symart_base::{DrawResponse, SymmetryChoice, make_layers_n, schema};

#[derive(Clone)]
pub struct SquigglesParam {
    pub exponent: f64,
    pub alpha: f64,
    pub thickness: f64,
    pub sharpness: f64
}

fn generate_single<R: Rng + ?Sized>(rng: &mut R, alpha: f64) -> Complex64 {
    Complex64::new(Levy { alpha }.sample(rng), 0.)
}

fn generate_double<R: Rng + ?Sized>(rng: &mut R, alpha: f64) -> Complex64 {
    Complex64::new(Levy { alpha }.sample(rng), Levy { alpha }.sample(rng))
}

fn generate_noise<R: Rng + ?Sized>(plan: &Plan2D, rng: &mut R, alpha: f64, single: bool) -> Array2<Complex64>
{
    let gen = if single { generate_single } else { generate_double };
    Array2::from_shape_fn((plan.width(), plan.height()), |_| gen(rng, alpha))
}

fn generate_noise_symmetric<R: Rng + ?Sized>(plan: &Plan2D, rng: &mut R, alpha: f64, single: bool, sym: SymmetryGroup) -> Array2<Complex64>
{
    let gen = if single { generate_single } else { generate_double };
    let mut arr = Array2::zeros((plan.width(), plan.height()));
    let dims = WrapDimension::new_from_shape(arr.shape());
    let transforms = transformations(sym, (plan.width() / 2) as i32);
    for (x, y) in indices_of(&arr) {
        let pt = Coord::new(x as i32, y as i32);
        let v = gen(rng, alpha);
        for t in &transforms {
            arr[dims.compute_index(&t.apply(&pt))] += v;
        }
    }
    arr
}

fn convolve(plan: &Plan2D, arr: &mut Array2<Complex64>, exponent: f64) {
    plan.apply(arr);
    let ax = 2. * PI / (plan.width() as f64);
    let ay = 2. * PI / (plan.height() as f64);
    let c = 3. - f64::cos(f64::min(ax, ay));
    for ((x, y), d) in arr.indexed_iter_mut() {
        let r = c - f64::cos(ax * (x as f64)) - f64::cos(ay * (y as f64));
        *d *= r.powf(-exponent/2.0);
    }
    plan.apply(arr);
}

fn make_squiggles<F>(arr: &Array2<Complex64>, mut proj: F, thickness: f64, sharpness: f64) -> Array2<u8>
where
    F: FnMut(&Complex64) -> f64
{
    let n2: f64 = arr.iter().map(&mut proj).map(|x| x*x).sum();
    let norm = 6.4 / (thickness * (n2 / (arr.len() as f64)).sqrt());
    Array2::from_shape_vec(arr.raw_dim(), arr.iter().map(|p| {
        let height = (proj(p) * norm).abs();
        (255.99 / (height.powf(sharpness) + 1.)) as u8
    }).collect()).unwrap()
}

fn make_squiggles_symmetric<F>(arr: &Array2<Complex64>, proj: F, thickness: f64, sharpness: f64, sg: SymmetryGroup) -> SymmetricCanvas<u8>
where
    F: FnMut(&Complex64) -> f64
{
    let wc: WrapCanvas<u8> = make_squiggles(arr, proj, thickness, sharpness).into();
    SymmetricCanvas::from_wrap_canvas(wc, sg)
}

fn proj_re(c: &Complex64) -> f64 { c.re }

fn proj_im(c: &Complex64) -> f64 { c.im }

pub fn generate_squiggles(plan: &Plan2D, param: &SquigglesParam, single: bool) -> Vec<Array2<u8>> {
    let mut arr = sample_fn(|rng| generate_noise(plan, rng, param.alpha, single));
    convolve(plan, &mut arr, param.exponent);
    let n = if single {1} else {2};
    [proj_re, proj_im][..n].iter().map(|f| make_squiggles(&arr, f, param.thickness, param.sharpness)).collect()
}

pub fn generate_squiggles_symmetric(sym: SymmetryGroup, plan: &Plan2D, param: &SquigglesParam, single: bool) -> Vec<SymmetricCanvas<u8>> {
    let mut arr = sample_fn(|rng| generate_noise_symmetric(plan, rng, param.alpha, single, sym));
    convolve(plan, &mut arr, param.exponent);
    let n = if single {1} else {2};
    [proj_re, proj_im][..n].iter().map(|f| make_squiggles_symmetric(&arr, f, param.thickness, param.sharpness, sym)).collect()
}

pub fn squiggles_layers(n: usize, plan: &Plan2D, param: &SquigglesParam) -> impl Iterator<Item = Array2<u8>> {
    let pl = plan.clone();
    let pa = param.clone();
    make_layers_n((n+1)/2, move |i| generate_squiggles(&pl, &pa, 2*i == n-1)).flat_map(|l| l.into_iter())
}

pub fn squiggles_layers_symmetric(n: usize, sym: SymmetryGroup, plan: &Plan2D, param: &SquigglesParam) -> impl Iterator<Item = SymmetricCanvas<u8>> {
    let pl = plan.clone();
    let pa = param.clone();
    make_layers_n((n+1)/2, move |i| generate_squiggles_symmetric(sym, &pl, &pa, 2*i == n-1)).flat_map(|l| l.into_iter())
}

#[derive(Deserialize)]
pub struct Squiggles {
    pub symmetry: SymmetryChoice,
    pub size: u32,
    pub colors: usize,
    pub exponent: f64,
    pub alpha: f64,
    pub thickness: f64,
    pub sharpness: f64
}

impl symart_base::Design for Squiggles {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "title": "Parameters",
            "type": "object",
            "properties": {
                "symmetry": schema::symmetries(),
                "size": schema::size_even(),
                "colors": schema::num_colors(),
                "exponent": {
                    "type": "number",
                    "title": "Exponent",
                    "default": 2
                },
                "alpha": {
                    "type": "number",
                    "title": "Alpha",
                    "minimum": 0.01,
                    "maximum": 2,
                    "default": 2
                },
                "thickness": {
                    "type": "number",
                    "title": "Thickness",
                    "default": 1
                },
                "sharpness": {
                    "type": "number",
                    "title": "Alpha",
                    "default": 2
                }
            },
            "required": ["symmetry", "size", "colors", "alpha", "thickness", "sharpness"]
        })
    }

    fn draw(&self) -> DrawResponse {
        let sym: SymmetryGroup = self.symmetry.into();
        let mut im = RgbImage::new(self.size, self.size);
        let param = SquigglesParam {
            exponent: self.exponent,
            alpha: self.alpha,
            thickness: self.thickness,
            sharpness: self.sharpness
        };
        let plan = Plan2D::new(&mut FFTplanner::new(false), self.size as usize, self.size as usize);
        squiggles_layers_symmetric(self.colors, sym, &plan, &param).for_each(|layer| {
            let col = symart_base::rng::sample(symart_base::random::Color);
            symart_base::layer::merge_one(&mut im, layer.as_ref(), image::Rgb(col));
        });
        DrawResponse { im, sym }
    }
}
