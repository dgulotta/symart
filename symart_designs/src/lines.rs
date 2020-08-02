use alga::general::AdditiveGroup;
use na::{Point2, Scalar, Vector2};
use num_traits::identities::zero;
use image::RgbImage;
use ordered_float::NotNan;
use rand::distributions::uniform::{SampleUniform, Uniform};
use rand::Rng;
use rand_distr::{Bernoulli, Cauchy, Distribution, Exp1, Poisson, StandardNormal};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};
use strum_macros::{Display, EnumCount, EnumIter, EnumString, IntoStaticStr};

use symart_base::canvas::Coord;
use symart_base::symmetric_canvas::SymmetricCanvas;
use symart_base::symmetry::{GridNorm, SymmetryGroup};
use symart_base::{DrawResponse, SymmetryChoice, schema};

struct NormalDist(pub GridNorm);
struct NormalScaled(f64);

impl Distribution<f64> for NormalScaled {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let n: f64 = StandardNormal.sample(rng);
        self.0 * n
    }
}

impl Distribution<Vector2<f64>> for NormalDist {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2<f64> {
        match self.0 {
            GridNorm::Square => {
                Vector2::new(StandardNormal.sample(rng), StandardNormal.sample(rng))
            }
            GridNorm::Hexagonal => {
                let s = NormalScaled(0.93060485910209959893).sample(rng);
                let d = NormalScaled(0.53728496591177095978).sample(rng);
                Vector2::new(s + d, s - d)
            }
        }
    }
}

struct PointDist {
    pub size: i32,
}

impl<T> Distribution<Point2<T>> for PointDist
where
    T: SampleUniform + Scalar + From<i32>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point2<T> {
        let x = Uniform::new(T::from(0), T::from(self.size)).sample(rng);
        let y = Uniform::new(T::from(0), T::from(self.size)).sample(rng);
        Point2::new(x, y)
    }
}

struct EndpointDist {
    pub size: i32,
    pub wrap: i32,
}

impl<T> Distribution<(Point2<T>, Point2<T>)> for EndpointDist
where
    T: SampleUniform + Scalar + From<i32> + AdditiveGroup + Copy,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (Point2<T>, Point2<T>) {
        let pt = PointDist { size: self.size }.sample(rng);
        let xwrap = Uniform::new_inclusive(-self.wrap, self.wrap).sample(rng);
        let ywrap = Uniform::new_inclusive(-self.wrap, self.wrap).sample(rng);
        (
            pt,
            pt + Vector2::new(T::from(self.size * xwrap), T::from(self.size * ywrap)),
        )
    }
}

type CoordF = Point2<f64>;

fn to_float(c: Coord) -> CoordF {
    CoordF::new(c.x as f64, c.y as f64)
}

fn from_float(c: CoordF) -> Coord {
    Coord::new(c.x.round() as i32, c.y.round() as i32)
}

fn midpoint(p1: &CoordF, p2: &CoordF) -> CoordF {
    CoordF::new(0.5 * (p1.x + p2.x), 0.5 * (p1.y + p2.y))
}

fn unit_vector(q: f64) -> Vector2<f64> {
    Vector2::new(q.cos(), q.sin())
}

pub struct LayerGenerator<'a, 'b, R: Rng + ?Sized + 'b> {
    pub canvas: &'a mut SymmetricCanvas<u8>,
    pub rng: &'b mut R,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    EnumCount,
    EnumString,
    IntoStaticStr,
)]
pub enum Design {
    Arc,
    Beads,
    Cluster,
    Curl,
    Flower,
    Fractal,
    Granules,
    Line,
    Loop,
    Orbit,
    Star,
    String,
    Swirl,
    Tree,
}

const SWIRL_EPS: f64 = 0.005;
const SQRT3: f64 = 0.86602540378443864676;

struct StarPathNode {
    pub dist: NotNan<f64>,
    pub pos: Coord,
}

impl PartialEq for StarPathNode {
    fn eq(&self, other: &StarPathNode) -> bool {
        self.dist == other.dist
    }
}

impl Eq for StarPathNode {}

impl PartialOrd for StarPathNode {
    fn partial_cmp(&self, other: &StarPathNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StarPathNode {
    fn cmp(&self, other: &StarPathNode) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl<'a, 'b, R: Rng + ?Sized + 'b> LayerGenerator<'a, 'b, R> {
    pub fn new(canvas: &'a mut SymmetricCanvas<u8>, rng: &'b mut R) -> Self {
        Self { canvas, rng }
    }

    fn norm_type(&self) -> GridNorm {
        GridNorm::from_symmetry(self.canvas.symmetry_group())
    }

    fn norm(&self, v: &Vector2<f64>) -> f64 {
        self.norm_type().norm(v)
    }

    fn random_normal(&mut self) -> Vector2<f64> {
        NormalDist(self.norm_type()).sample(self.rng)
    }

    fn size(&self) -> usize {
        self.canvas.size()
    }

    fn symmetry_group(&self) -> SymmetryGroup {
        self.canvas.symmetry_group()
    }

    fn num_symmetries(&self) -> usize {
        self.symmetry_group().num_symmetries()
    }

    fn draw_pixel(&mut self, pt: &Coord, intensity: u8) {
        if intensity > self.canvas[*pt] {
            self.canvas.set(pt, intensity);
        }
    }

    fn draw_dot_default(&mut self, pt: &CoordF) {
        self.draw_dot(pt, 5, 1.);
    }

    fn draw_dot(&mut self, pt: &CoordF, radius: i32, brightness: f64) {
        let denom = brightness
            * match self.norm_type() {
                GridNorm::Square => 5.,
                GridNorm::Hexagonal => 4.33,
            };
        let num = denom * 255.;
        let closest = from_float(*pt);
        for x in (closest.x - radius)..=(closest.x + radius) {
            for y in (closest.y - radius)..=(closest.y + radius) {
                let pix = Point2::new(x, y);
                let d = self.norm(&(pt - to_float(pix)));
                self.draw_pixel(&pix, (num / (denom + d)) as u8);
            }
        }
    }

    fn draw_smooth_arc(&mut self, p1: &CoordF, p2: &CoordF, curvature: f64, stdev: f64, dist: f64) {
        let mx = (p1.x + p2.x) / 2. + curvature * (p2.y - p1.y);
        let my = (p1.y + p2.y) / 2. + curvature * (p1.x - p2.x);
        let off = stdev * self.random_normal();
        let mid = CoordF::new(mx, my) + off;
        self.draw_dot_default(&mid);
        if self.norm(&(mid - p1)) >= dist {
            self.draw_smooth_arc(p1, &mid, curvature / 2., stdev / 2., dist);
        }
        if self.norm(&(mid - p2)) >= dist {
            self.draw_smooth_arc(&mid, p2, curvature / 2., stdev / 2., dist);
        }
    }

    fn draw_line(&mut self, p1: &CoordF, p2: &CoordF, stdev: f64, dist: f64, factor: f64) {
        let mid = midpoint(p1, p2) + stdev * self.random_normal();
        self.draw_dot_default(&mid);
        if self.norm(&(mid - p1)) >= dist {
            self.draw_line(p1, &mid, stdev * factor, dist, factor);
        }
        if self.norm(&(mid - p2)) >= dist {
            self.draw_line(&mid, p2, stdev * factor, dist, factor);
        }
    }

    fn draw_smooth_line(&mut self, p1: &CoordF, p2: &CoordF, stdev: f64, dist: f64) {
        self.draw_line(p1, p2, stdev, dist, 0.5);
    }

    fn draw_smooth_line_new(
        &mut self,
        p1: &CoordF,
        v1: &Vector2<f64>,
        p2: &CoordF,
        v2: &Vector2<f64>,
        stdev: f64,
        dist: f64,
    ) {
        let mid = midpoint(p1, p2) + 0.25 * (v1 - v2) + stdev * self.random_normal();
        let vmid =
            0.375 * (p2 - p1) - 0.125 * (v1 + v2) + stdev * self.random_normal() * (0.5 * SQRT3);
        self.draw_dot_default(&mid);
        let new_stdev = stdev * (0.25 * SQRT_2);
        if self.norm(&(mid - p1)) >= dist {
            self.draw_smooth_line_new(p1, &(0.5 * v1), &mid, &vmid, new_stdev, dist);
        }
        if self.norm(&(mid - p1)) >= dist {
            self.draw_smooth_line_new(&mid, &vmid, p2, &(0.5 * v2), new_stdev, dist);
        }
    }

    fn random_endpoints<T>(&mut self, wrap: i32) -> (Point2<T>, Point2<T>)
    where
        T: SampleUniform + Scalar + From<i32> + AdditiveGroup + Copy,
    {
        EndpointDist {
            size: self.size() as i32,
            wrap,
        }
        .sample(self.rng)
    }

    fn random_point<T>(&mut self) -> Point2<T>
    where
        T: SampleUniform + Scalar + From<i32>,
    {
        PointDist {
            size: self.size() as i32,
        }
        .sample(self.rng)
    }

    fn draw_cluster(&mut self, center: &CoordF, stdev: f64, max_depth: usize) {
        let pt = center + stdev * self.random_normal();
        let clusters = Uniform::new(0, max_depth).sample(self.rng);
        for _ in 0..clusters {
            self.draw_smooth_line(center, &pt, stdev / 2., 1.);
            self.draw_cluster(&pt, stdev / 2., max_depth - 1);
        }
        self.draw_dot_default(&center);
    }

    fn draw_flower(&mut self, center: &CoordF, steps: usize) {
        let e: f32 = Exp1.sample(self.rng);
        let petals = 3 + ((10. * e) as usize);
        let offset = Uniform::new(0., 1.).sample(self.rng);
        let angle_mult = 2. * PI / (petals as f64);
        for i in 0..petals {
            let mut pt = *center;
            let angle = angle_mult * ((i as f64) + offset) / (petals as f64);
            let mut v = unit_vector(angle);
            v += 0.07 * self.random_normal();
            for _ in 0..steps {
                v += 0.07 * self.random_normal();
                pt += v;
                self.draw_dot_default(&pt);
            }
        }
    }

    fn fractal_prob(&self) -> u8 {
        match self.num_symmetries() {
            1 | 2 => 40,
            3 => 38,
            4 => 37,
            6 => 35,
            8 => 34,
            12 => 32,
            _ => unreachable!(),
        }
    }

    fn draw_fractal(&mut self, center: &Coord, size: u32, prob: u8) {
        let newsize = size / 2;
        if newsize != 0 {
            let dist = Uniform::new(0, 60);
            for dx in [0, newsize as i32].iter() {
                for dy in [0, newsize as i32].iter() {
                    if dist.sample(self.rng) < prob {
                        self.draw_fractal(&Coord::new(center.x + dx, center.y + dy), newsize, prob);
                    }
                }
            }
        } else {
            self.draw_dot_default(&to_float(*center));
        }
    }

    fn draw_swirl(&mut self) {
        let len = Exp1.sample(self.rng);
        let k1 = NormalScaled(3.).sample(self.rng);
        let k2 = NormalScaled(3.).sample(self.rng);
        let mut q = Uniform::new(0., 2. * PI).sample(self.rng);
        let mut pt = self.random_point();
        let mut t = 0.;
        while t < len {
            self.draw_dot_default(&pt);
            q += SWIRL_EPS * (t * k1 + (1. - t) * k2) / len;
            pt += unit_vector(q);
            t += SWIRL_EPS;
        }
    }

    fn draw_tree(&mut self, start: &CoordF, q: f64, depth: usize) {
        let mut pt = *start;
        let n = Distribution::<u64>::sample(&Poisson::new(20.).unwrap(),self.rng) as usize;
        let v = unit_vector(q);
        for _ in 0..n {
            pt += v;
            self.draw_dot_default(&pt);
        }
        let d = depth + n;
        let p = 1. / (1. + (d as f64) / 100.);
        if Bernoulli::new(p).unwrap().sample(self.rng) {
            self.draw_tree_split(&pt, q, d);
        }
    }

    fn draw_tree_split(&mut self, pt: &CoordF, q: f64, depth: usize) {
        let dq = NormalScaled(PI / 6.).sample(self.rng);
        self.draw_tree(pt, q + dq, depth);
        self.draw_tree(pt, q - dq, depth);
    }

    pub fn generate(&mut self, design: Design) {
        use self::Design::*;
        match design {
            Arc => {
                let (start, end) = self.random_endpoints(2);
                self.draw_smooth_arc(&start, &end, 0.8, 30., 1.);
            }
            Beads => {
                let (start, end) = self.random_endpoints(2);
                self.draw_smooth_line(&start, &end, 100., 100.);
            }
            Cluster => {
                let pt = self.random_point();
                self.draw_cluster(&pt, 40., 4);
            }
            Curl => self.draw_curl(),
            Flower => {
                let pt = self.random_point();
                self.draw_flower(&pt, 50);
            }
            Fractal => {
                let pt = self.random_point();
                let sz = self.size() as u32;
                let fp = self.fractal_prob();
                self.draw_fractal(&pt, sz, fp);
            }
            Granules => self.draw_granules(),
            Line => {
                let (start, end) = self.random_endpoints(2);
                self.draw_smooth_line(&start, &end, 100., 1.);
            }
            Loop => {
                let (p1, p2) = self.random_endpoints(1);
                let v = (self.size() as f64) * self.random_normal();
                self.draw_smooth_line_new(&p1, &v, &p2, &v, 200., 1.);
            }
            Orbit => self.draw_orbit(),
            Star => self.draw_star(),
            String => {
                let e: f64 = Exp1.sample(self.rng);
                let sigma = e * (self.size() as f64) * 0.07;
                let p1 = self.random_point();
                let p2 = p1 + sigma * self.random_normal();
                self.draw_line(&p1, &p2, sigma / 2., 1., FRAC_1_SQRT_2);
            }
            Swirl => loop {
                self.draw_swirl();
                if Uniform::new(0, 20).sample(self.rng) < self.num_symmetries() {
                    break;
                }
            },
            Tree => {
                let pt = self.random_point();
                let q = Uniform::new(0., 2. * PI).sample(self.rng);
                self.draw_tree_split(&pt, q, 0);
            }
        }
    }

    fn draw_granules(&mut self) {
        let mut pt = self.random_point();
        let mu = ((self.size() * self.size()) as f64) / (10. * (self.num_symmetries() as f64));
        let steps = Poisson::new(mu).unwrap().sample(self.rng);
        for _ in 0..steps {
            let z = NormalScaled(3.).sample(self.rng);
            pt += self.random_normal() / z;
            self.draw_dot_default(&pt);
        }
    }

    fn draw_curl(&mut self) {
        let mut pt = self.random_point();
        let mut q = Uniform::new(0., 2. * PI).sample(self.rng);
        let mut dq = Cauchy::new(0., 0.167).unwrap().sample(self.rng);
        let steps = Poisson::new(2500.).unwrap().sample(self.rng);
        for _ in 0..steps {
            dq *= 0.97;
            dq += Cauchy::new(0., 0.005).unwrap().sample(self.rng);
            q += dq;
            pt += unit_vector(q);
            self.draw_dot_default(&pt);
        }
    }

    fn draw_orbit(&mut self) {
        let mut q = {
            let d = Uniform::new(0., 2. * PI);
            let mut f = || Point2::new(d.sample(self.rng), d.sample(self.rng));
            [f(), f(), f()]
        };
        let mut dq = {
            let mut f = || 0.78 * self.random_normal();
            [f(), f(), f()]
        };
        let scale = (self.size() as f64) / (2. * PI);
        let mut t = 12. / (self.num_symmetries() as f64);
        while t >= 0. {
            for pt in q.iter() {
                self.draw_dot_default(&(scale * pt));
            }
            for i in 1..3 {
                for j in 0..i {
                    let diff = q[i] - q[j];
                    let d = 2. - diff[0].cos() - diff[1].cos();
                    let s = (SWIRL_EPS / d) * Vector2::new(diff[0].sin(), diff[1].sin());
                    dq[i] -= s;
                    dq[j] += s;
                }
            }
            for i in 0..3 {
                q[i] += SWIRL_EPS * dq[i];
            }
            t -= SWIRL_EPS;
        }
    }

    fn draw_star(&mut self) {
        let hsz = (self.size() / 2) as u32;
        let (hdist, vdist) = {
            let mut f =
                || SymmetricCanvas::from_fn(self.symmetry_group(), hsz, || Exp1.sample(self.rng));
            (f(), f())
        };
        let mut mark = SymmetricCanvas::<bool>::new(self.symmetry_group(), hsz);
        let starsize = (self.size() as f64) / 15.;
        let mut queue = BinaryHeap::new();
        for _ in 0..(Poisson::new(5.).unwrap().sample(self.rng)) {
            queue.push(StarPathNode {
                dist: zero(),
                pos: self.random_point(),
            });
        }
        while let Some(p) = queue.pop() {
            if !mark[p.pos] {
                if p.dist.into_inner() >= starsize {
                    break;
                }
                mark.set(&p.pos, true);
                self.draw_pixel(&p.pos, (256. * (1. - p.dist.into_inner() / starsize)) as u8);
                let mut try_push = |disp, arr: &SymmetricCanvas<f64>, adisp| {
                    let pnew = p.pos + disp;
                    if !mark[pnew] {
                        queue.push(StarPathNode {
                            dist: p.dist + NotNan::new(arr[p.pos + adisp]).unwrap(),
                            pos: pnew,
                        });
                    }
                };
                try_push(Vector2::new(-1, 0), &hdist, Vector2::new(0, 0));
                try_push(Vector2::new(1, 0), &hdist, Vector2::new(1, 0));
                try_push(Vector2::new(0, -1), &vdist, Vector2::new(0, 0));
                try_push(Vector2::new(0, 1), &vdist, Vector2::new(0, 1));
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Lines {
    pub symmetry: SymmetryChoice,
    pub size: u32,
    pub colors: usize,
    pub designs: Vec<Design>,
}

pub fn lines_designs() -> serde_json::Value {
    let v = schema::enum_strings::<Design>();
    let default = v[0].clone();
    serde_json::json!({
        "type": "array",
        "title": "Designs",
        "minItems": 1,
        "items": {
            "type": "string",
            "enum": v
        },
        "default": [ default ]
    })
}

impl symart_base::Design for Lines {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "title": "Parameters",
            "type": "object",
            "properties": {
                "symmetry": schema::symmetries(),
                "size": schema::size_even(),
                "colors": schema::num_colors(),
                "designs": lines_designs()
            },
            "required": ["size", "symmetry", "colors", "designs"]
        })
    }

    fn draw(&self) -> DrawResponse {
        let sym: SymmetryGroup = self.symmetry.into();
        let mut im = RgbImage::new(self.size, self.size);
        symart_base::make_layers(self.colors, || {
            let mut canvas = SymmetricCanvas::new(sym, self.size / 2);
            symart_base::rng::sample_fn(|rng| {
                let idx = Uniform::new(0, self.designs.len()).sample(rng);
                let design = self.designs[idx];
                let mut lg = LayerGenerator {
                    canvas: &mut canvas,
                    rng,
                };
                lg.generate(design);
            });
            canvas
        })
        .for_each(|layer| {
            let col = symart_base::rng::sample(symart_base::random::Color);
            symart_base::layer::merge_one(&mut im, layer.as_ref(), image::Rgb(col));
        });
        DrawResponse { im, sym }
    }
}
