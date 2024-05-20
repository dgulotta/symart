use image::RgbImage;
use na::{Matrix4x2, Matrix6, Vector2, Vector4, Vector6};
use nalgebra as na;
use num_complex::Complex64;
use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Uniform;
use std::f64::consts::FRAC_1_SQRT_2;
use std::f64::consts::PI;
use symart_base::random::{ComplexStdNormal, Fraction, NormalScaled, Slice};
use symart_base::{schema, DrawResponse};
use thiserror::Error;

type V4 = nalgebra::Matrix<f64, na::U4, na::U1, na::ArrayStorage<f64, 4, 1>>;
type V2 = nalgebra::Matrix<f64, na::U2, na::U1, na::ArrayStorage<f64, 2, 1>>;

fn zero(_: V4) -> V4 {
    Vector4::new(0.0, 0.0, 0.0, 0.0)
}
fn ident(v: V4) -> V4 {
    v
}
fn invert(v: V4) -> V4 {
    -v
}

fn rot10_1(v: V4) -> V4 {
    Vector4::new(-v.y, -v.z, -v.w, v.x + v.y + v.z + v.w)
}
fn rot10_2(v: V4) -> V4 {
    Vector4::new(v.z, v.w, -(v.x + v.y + v.z + v.w), v.x)
}
fn rot10_3(v: V4) -> V4 {
    Vector4::new(-v.w, v.x + v.y + v.z + v.w, -v.x, -v.y)
}
fn rot10_4(v: V4) -> V4 {
    Vector4::new(-(v.x + v.y + v.z + v.w), v.x, v.y, v.z)
}
fn rot10_5(v: V4) -> V4 {
    Vector4::new(v.y, v.z, v.w, -(v.x + v.y + v.z + v.w))
}
fn rot10_6(v: V4) -> V4 {
    Vector4::new(-v.z, -v.w, v.x + v.y + v.z + v.w, -v.x)
}
fn rot10_7(v: V4) -> V4 {
    Vector4::new(v.w, -(v.x + v.y + v.z + v.w), v.x, v.y)
}
fn rot10_8(v: V4) -> V4 {
    Vector4::new(v.x + v.y + v.z + v.w, -v.x, -v.y, -v.z)
}

fn flip10_1(v: V4) -> V4 {
    Vector4::new(v.y, v.w, v.x, v.z)
}
fn flip10_2(v: V4) -> V4 {
    Vector4::new(v.z, v.x, v.w, v.y)
}
fn flip10_3(v: V4) -> V4 {
    Vector4::new(v.w, v.z, v.y, v.x)
}

fn flip8_1(v: V4) -> V4 {
    Vector4::new(v.x, v.w, -v.z, v.y)
}
fn flip8_2(v: V4) -> V4 {
    Vector4::new(v.x, -v.y, v.z, -v.w)
}
fn flip8_3(v: V4) -> V4 {
    Vector4::new(v.x, -v.w, -v.z, -v.y)
}

fn flip12_1(v: V4) -> V4 {
    Vector4::new(v.x, v.w - v.y, v.x - v.z, v.w)
}
fn flip12_2(v: V4) -> V4 {
    Vector4::new(v.x, -v.y, v.z, -v.w)
}
fn flip12_3(v: V4) -> V4 {
    Vector4::new(v.x, v.y - v.w, v.x - v.z, -v.w)
}

fn rot8_1(v: V4) -> V4 {
    Vector4::new(v.y, v.z, v.w, -v.x)
}
fn rot8_2(v: V4) -> V4 {
    Vector4::new(v.z, v.w, -v.x, -v.y)
}
fn rot8_3(v: V4) -> V4 {
    Vector4::new(v.w, -v.x, -v.y, -v.z)
}
fn rot8_4(v: V4) -> V4 {
    Vector4::new(-v.y, -v.z, -v.w, v.x)
}
fn rot8_5(v: V4) -> V4 {
    Vector4::new(-v.z, -v.w, v.x, v.y)
}
fn rot8_6(v: V4) -> V4 {
    Vector4::new(-v.w, v.x, v.y, v.z)
}

fn rot12_1(v: V4) -> V4 {
    Vector4::new(v.y, v.z, v.w, v.z - v.x)
}
fn rot12_2(v: V4) -> V4 {
    Vector4::new(v.z, v.w, v.z - v.x, v.w - v.y)
}
fn rot12_3(v: V4) -> V4 {
    Vector4::new(v.w, v.z - v.x, v.w - v.y, -v.x)
}
fn rot12_4(v: V4) -> V4 {
    Vector4::new(v.z - v.x, v.w - v.y, -v.x, -v.y)
}
fn rot12_5(v: V4) -> V4 {
    Vector4::new(v.w - v.y, -v.x, -v.y, -v.z)
}
fn rot12_6(v: V4) -> V4 {
    Vector4::new(-v.y, -v.z, -v.w, v.x - v.z)
}
fn rot12_7(v: V4) -> V4 {
    Vector4::new(-v.z, -v.w, v.x - v.z, v.y - v.w)
}
fn rot12_8(v: V4) -> V4 {
    Vector4::new(-v.w, v.x - v.z, v.y - v.w, v.x)
}
fn rot12_9(v: V4) -> V4 {
    Vector4::new(v.x - v.z, v.y - v.w, v.x, v.y)
}
fn rot12_10(v: V4) -> V4 {
    Vector4::new(v.y - v.w, v.x, v.y, v.z)
}

static TRANSFORMS_5: [fn(V4) -> V4; 11] = [
    zero, ident, invert, rot10_1, rot10_2, rot10_3, rot10_4, rot10_5, rot10_6, rot10_7, rot10_8,
];
static TRANSFORMS_8: [fn(V4) -> V4; 9] = [
    zero, ident, invert, rot8_1, rot8_2, rot8_3, rot8_4, rot8_5, rot8_6,
];
static TRANSFORMS_12: [fn(V4) -> V4; 13] = [
    zero, ident, invert, rot12_1, rot12_2, rot12_3, rot12_4, rot12_5, rot12_6, rot12_7, rot12_8,
    rot12_9, rot12_10,
];
static FLIPS_5: [fn(V4) -> V4; 4] = [ident, flip10_1, flip10_2, flip10_3];
static FLIPS_8: [fn(V4) -> V4; 4] = [ident, flip8_1, flip8_2, flip8_3];
static FLIPS_12: [fn(V4) -> V4; 4] = [ident, flip12_1, flip12_2, flip12_3];

trait TrapRunner {
    type Point;
    fn new_random<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn embed(&self, v: Vector2<f64>) -> Self::Point;
    fn iterate(&self, p: Self::Point) -> Self::Point;
    fn dist(&self, p: Self::Point) -> f64;
    fn num_iters(&self) -> usize;
    fn run(&self, v2: Vector2<f64>) -> u8 {
        let mut v = self.embed(v2);
        for _ in 0..self.num_iters() {
            v = self.iterate(v)
        }
        let dm = self.dist(v);
        (127.999 * (dm + 1.0)) as u8
    }
}

struct Trap5Trig {
    a0: f64,
    a1: fn(V4) -> V4,
    a3: Complex64,
    a4: Complex64,
    a5: Complex64,
    a6: Complex64,
    a7: Complex64,
    flip: fn(V4) -> V4,
    offset: V4,
}

impl TrapRunner for Trap5Trig {
    type Point = V4;
    fn new_random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            a0: Fraction { denom: 5 }.sample(rng),
            a1: Slice {
                slice: &TRANSFORMS_5,
            }
            .sample(rng),
            a3: 0.2 * ComplexStdNormal.sample(rng),
            a4: 0.2 * ComplexStdNormal.sample(rng),
            a5: 0.2 * ComplexStdNormal.sample(rng),
            a6: 0.2 * ComplexStdNormal.sample(rng),
            a7: 0.2 * ComplexStdNormal.sample(rng),
            flip: Slice { slice: &FLIPS_5 }.sample(rng),
            offset: Offset.sample(rng),
        }
    }
    fn embed(&self, v: Vector2<f64>) -> V4 {
        emb_5() * v + self.offset
    }
    fn iterate(&self, v: V4) -> V4 {
        let ex = Complex64::from_polar(1.0, v.x);
        let ey = Complex64::from_polar(1.0, v.y);
        let ez = Complex64::from_polar(1.0, v.z);
        let ew = Complex64::from_polar(1.0, v.w);
        let ev = (ex * ey * ez * ew).conj();
        let xn = (self.a3 * ex + self.a4 * ey + self.a5 * ez + self.a6 * ew + self.a7 * ev).re;
        let yn = (self.a7 * ex + self.a3 * ey + self.a4 * ez + self.a5 * ew + self.a6 * ev).re;
        let zn = (self.a6 * ex + self.a7 * ey + self.a3 * ez + self.a4 * ew + self.a5 * ev).re;
        let wn = (self.a5 * ex + self.a6 * ey + self.a7 * ez + self.a3 * ew + self.a4 * ev).re;
        let vn = (self.a4 * ex + self.a5 * ey + self.a6 * ez + self.a7 * ew + self.a3 * ev).re;
        let sn = self.a0 - 0.2 * (xn + yn + zn + wn + vn);
        let vecn = Vector4::new(xn + sn, yn + sn, zn + sn, wn + sn) + (self.a1)(v);
        (self.flip)(vecn)
    }
    fn dist(&self, p: V4) -> f64 {
        dist_5(p)
    }
    fn num_iters(&self) -> usize {
        15
    }
}

struct Trap10Trig {
    a1: fn(V4) -> V4,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    flip: fn(V4) -> V4,
    offset: V4,
}

impl TrapRunner for Trap10Trig {
    type Point = V4;
    fn new_random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            a1: Slice {
                slice: &TRANSFORMS_5,
            }
            .sample(rng),
            a3: NormalScaled(0.5).sample(rng),
            a4: NormalScaled(0.5).sample(rng),
            a5: NormalScaled(0.5).sample(rng),
            a6: NormalScaled(0.5).sample(rng),
            a7: NormalScaled(0.5).sample(rng),
            flip: Slice { slice: &FLIPS_5 }.sample(rng),
            offset: Offset.sample(rng),
        }
    }
    fn embed(&self, v: Vector2<f64>) -> V4 {
        emb_5() * v + self.offset
    }
    fn iterate(&self, v: V4) -> V4 {
        let ex = v.x.sin();
        let ey = v.y.sin();
        let ez = v.z.sin();
        let ew = v.w.sin();
        let ev = -(v.x + v.y + v.z + v.w).sin();
        let xn = self.a3 * ex + self.a4 * ey + self.a5 * ez + self.a6 * ew + self.a7 * ev;
        let yn = self.a7 * ex + self.a3 * ey + self.a4 * ez + self.a5 * ew + self.a6 * ev;
        let zn = self.a6 * ex + self.a7 * ey + self.a3 * ez + self.a4 * ew + self.a5 * ev;
        let wn = self.a5 * ex + self.a6 * ey + self.a7 * ez + self.a3 * ew + self.a4 * ev;
        let vn = self.a4 * ex + self.a5 * ey + self.a6 * ez + self.a7 * ew + self.a3 * ev;
        let sn = -0.2 * (xn + yn + zn + wn + vn);
        let vecn = Vector4::new(xn + sn, yn + sn, zn + sn, wn + sn) + (self.a1)(v);
        (self.flip)(vecn)
    }
    fn dist(&self, p: V4) -> f64 {
        dist_5(p)
    }
    fn num_iters(&self) -> usize {
        15
    }
}

struct Trap8Trig {
    a0: f64,
    a1: fn(V4) -> V4,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    flip: fn(V4) -> V4,
    offset: V4,
}

impl TrapRunner for Trap8Trig {
    type Point = V4;
    fn new_random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            a0: Fraction { denom: 2 }.sample(rng),
            a1: Slice {
                slice: &TRANSFORMS_8,
            }
            .sample(rng),
            a3: NormalScaled(0.5).sample(rng),
            a4: NormalScaled(0.5).sample(rng),
            a5: NormalScaled(0.5).sample(rng),
            a6: NormalScaled(0.5).sample(rng),
            flip: Slice { slice: &FLIPS_8 }.sample(rng),
            offset: Offset.sample(rng),
        }
    }
    fn embed(&self, v: Vector2<f64>) -> V4 {
        emb_8() * v + self.offset
    }
    fn iterate(&self, v: V4) -> V4 {
        let ex = v.x.sin();
        let ey = v.y.sin();
        let ez = v.z.sin();
        let ew = v.w.sin();
        let xn = self.a0 + self.a3 * ex + self.a4 * ey + self.a5 * ez + self.a6 * ew;
        let yn = self.a0 + self.a3 * ey + self.a4 * ez + self.a5 * ew - self.a6 * ex;
        let zn = self.a0 + self.a3 * ez + self.a4 * ew - self.a5 * ex - self.a6 * ey;
        let wn = self.a0 + self.a3 * ew - self.a4 * ex - self.a5 * ey - self.a6 * ez;
        let vecn = Vector4::new(xn, yn, zn, wn) + (self.a1)(v);
        (self.flip)(vecn)
    }
    fn dist(&self, p: V4) -> f64 {
        dist_8(p)
    }
    fn num_iters(&self) -> usize {
        15
    }
}

struct Trap12Trig {
    a1: fn(V4) -> V4,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    a8: f64,
    flip: fn(V4) -> V4,
    offset: V4,
}

impl TrapRunner for Trap12Trig {
    type Point = V4;
    fn new_random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            a1: Slice {
                slice: &TRANSFORMS_12,
            }
            .sample(rng),
            a3: NormalScaled(0.5).sample(rng),
            a4: NormalScaled(0.5).sample(rng),
            a5: NormalScaled(0.5).sample(rng),
            a6: NormalScaled(0.5).sample(rng),
            a7: NormalScaled(0.5).sample(rng),
            a8: NormalScaled(0.5).sample(rng),
            flip: Slice { slice: &FLIPS_12 }.sample(rng),
            offset: Offset.sample(rng),
        }
    }
    fn embed(&self, v: Vector2<f64>) -> V4 {
        emb_12() * v + self.offset
    }
    fn iterate(&self, v: V4) -> V4 {
        let v6 = Vector6::new(
            v.x.sin(),
            v.y.sin(),
            v.z.sin(),
            v.w.sin(),
            (v.z - v.x).sin(),
            (v.w - v.y).sin(),
        );
        let m = Matrix6::new(
            self.a3, self.a4, self.a5, self.a6, self.a7, self.a8, -self.a8, self.a3, self.a4,
            self.a5, self.a6, self.a7, -self.a7, -self.a8, self.a3, self.a4, self.a5, self.a6,
            -self.a6, -self.a7, -self.a8, self.a3, self.a4, self.a5, -self.a5, -self.a6, -self.a7,
            -self.a8, self.a3, self.a4, -self.a4, -self.a5, -self.a6, -self.a7, -self.a8, self.a3,
        );
        let vn = m * v6;
        let sx = (1. / 3.) * (vn.x - vn.z + vn.a);
        let sy = (1. / 3.) * (vn.y - vn.w + vn.b);
        let vecn = Vector4::new(vn.x - sx, vn.y - sy, vn.z + sx, vn.w + sy) + (self.a1)(v);
        (self.flip)(vecn)
    }
    fn dist(&self, p: V4) -> f64 {
        dist_12(p)
    }
    fn num_iters(&self) -> usize {
        15
    }
}

fn emb_5() -> Matrix4x2<f64> {
    Matrix4x2::new(
        0.30901699437494745,
        0.9510565162951535,
        -0.8090169943749473,
        0.5877852522924732,
        -0.8090169943749473,
        -0.5877852522924732,
        0.30901699437494745,
        -0.9510565162951535,
    )
}

fn emb_8() -> Matrix4x2<f64> {
    Matrix4x2::new(
        1.0,
        0.0,
        FRAC_1_SQRT_2,
        FRAC_1_SQRT_2,
        0.0,
        1.0,
        -FRAC_1_SQRT_2,
        FRAC_1_SQRT_2,
    )
}

fn emb_12() -> Matrix4x2<f64> {
    Matrix4x2::new(
        1.0,
        0.0,
        0.8660254037844387,
        0.5,
        0.5,
        0.8660254037844387,
        0.0,
        1.0,
    )
}

fn dist_5(v: V4) -> f64 {
    0.2 * (v.x.cos() + v.y.cos() + v.z.cos() + v.w.cos() + (v.x + v.y + v.z + v.w).cos())
}

fn dist_8(v: V4) -> f64 {
    0.25 * (v.x.cos() + v.y.cos() + v.z.cos() + v.w.cos())
}

fn dist_12(v: V4) -> f64 {
    (1.0 / 6.0)
        * (v.x.cos() + v.y.cos() + v.z.cos() + v.w.cos() + (v.x - v.z).cos() + (v.y - v.w).cos())
}

struct Offset;

impl Distribution<V4> for Offset {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> V4 {
        let mut ang = || Uniform::new(0., 2. * PI).sample(rng);
        V4::new(ang(), ang(), ang(), ang())
    }
}

#[derive(Error, Debug)]
pub enum QuasitrapError {
    #[error("Bad parameters")]
    BadParam,
}

#[derive(Deserialize)]
pub struct Quasitrap {
    pub symmetries: u8,
    pub quasiperiod: f64,
    pub height: u32,
    pub width: u32,
}

fn make_runner<T: TrapRunner + 'static>() -> Box<dyn Fn(V2) -> u8> {
    let runner = symart_base::rng::sample_fn(|r| T::new_random(r));
    let f = move |v| runner.run(v);
    Box::new(f)
}

impl symart_base::Design for Quasitrap {
    fn name() -> &'static str {
        "Quasiperiodic Orbit Trap"
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "title": "Parameters",
            "type": "object",
            "properties": {
                "symmetries": {
                    "type": "integer",
                    "title": "Symmetries",
                    "enum": [5, 8, 10, 12],
                    "default": 5
                },
                "quasiperiod": {
                    "type": "number",
                    "title": "Quasiperiod",
                    "minimum": 1,
                    "default": 100
                },
                "height": schema::height(),
                "width": schema::width()
            },
            "required": ["symmetries", "quasiperiod"]
        })
    }

    fn draw(&self) -> Result<DrawResponse, Box<dyn std::error::Error>> {
        let runner = match self.symmetries {
            5 => make_runner::<Trap5Trig>(),
            8 => make_runner::<Trap8Trig>(),
            10 => make_runner::<Trap10Trig>(),
            12 => make_runner::<Trap12Trig>(),
            _ => return Err(Box::new(QuasitrapError::BadParam)),
        };
        let factor = 2. * PI / self.quasiperiod;
        let pixel_fn = move |x, y| {
            let v2 = factor * V2::new(x as f64, y as f64);
            let v = runner(v2);
            image::Rgb([v, v, v])
        };
        let im = RgbImage::from_fn(self.width, self.height, pixel_fn);
        Ok(DrawResponse {
            im,
            sym: symart_base::SymmetryType::None,
        })
    }
}
