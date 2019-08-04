use ndarray::Array2;
use std::ops::Index;

use crate::canvas::{Coord, WrapCanvas};
use crate::symmetry::{transformations, SymmetryGroup, Transformation};

pub struct SymmetricCanvas<T> {
    canvas: WrapCanvas<T>,
    transforms: Vec<Transformation<i32>>,
    group: SymmetryGroup,
}

impl<T> SymmetricCanvas<T> {
    pub fn symmetry_group(&self) -> SymmetryGroup {
        self.group
    }

    pub fn size(&self) -> usize {
        self.canvas.height()
    }

    fn from_wrap_canvas(canvas: WrapCanvas<T>, group: SymmetryGroup) -> Self {
        let hsz = (canvas.height() / 2) as i32;
        Self {
            canvas,
            transforms: transformations(group, hsz),
            group,
        }
    }
}

impl<T: Copy> SymmetricCanvas<T> {
    pub fn from_fn<F>(group: SymmetryGroup, hsz: u32, mut f: F) -> SymmetricCanvas<T>
    where
        F: FnMut() -> T,
    {
        let size = hsz * 2;
        let wc = unsafe { WrapCanvas::uninitialized(size, size) };
        let mut sc = Self::from_wrap_canvas(wc, group);
        for x in 0..(size as i32) {
            for y in 0..(size as i32) {
                let t = f();
                sc.set(&Coord::new(x, y), t);
            }
        }
        sc
    }
}

impl<T: Clone> SymmetricCanvas<T> {
    pub fn set(&mut self, idx: &Coord, t: T) {
        for tr in &self.transforms {
            self.canvas[tr.apply(idx)] = t.clone();
        }
    }

    pub fn from_elem(group: SymmetryGroup, hsz: u32, t: T) -> Self {
        let size = hsz * 2;
        Self::from_wrap_canvas(WrapCanvas::from_elem(size, size, t), group)
    }
}

impl<T: Default> SymmetricCanvas<T> {
    pub fn new(group: SymmetryGroup, hsz: u32) -> Self {
        let size = hsz * 2;
        Self::from_wrap_canvas(WrapCanvas::new(size, size), group)
    }
}

impl<T> AsRef<WrapCanvas<T>> for SymmetricCanvas<T> {
    fn as_ref(&self) -> &WrapCanvas<T> {
        &self.canvas
    }
}

impl<T> AsRef<Array2<T>> for SymmetricCanvas<T> {
    fn as_ref(&self) -> &Array2<T> {
        self.canvas.as_ref()
    }
}

impl<T> Into<WrapCanvas<T>> for SymmetricCanvas<T> {
    fn into(self) -> WrapCanvas<T> {
        self.canvas
    }
}

impl<T> Into<Array2<T>> for SymmetricCanvas<T> {
    fn into(self) -> Array2<T> {
        self.canvas.into()
    }
}

impl<T> Index<Coord> for SymmetricCanvas<T> {
    type Output = T;
    fn index(&self, idx: Coord) -> &T {
        &self.canvas[idx]
    }
}
