use na::Point2;
use ndarray::Array2;
use std::ops::Index;
use std::ops::IndexMut;

pub type Coord = Point2<i32>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Modulus {
    modulus: i32,
}

impl Modulus {
    pub fn new(modulus: i32) -> Self {
        Modulus { modulus: modulus }
    }

    pub fn apply(&self, n: i32) -> i32 {
        let m = n % self.modulus;
        if m >= 0 {
            m
        } else {
            m + self.modulus
        }
    }
}

pub struct WrapDimension {
    vert: Modulus,
    horiz: Modulus
}

impl WrapDimension {
    pub fn new(vert: i32, horiz: i32) -> Self {
        WrapDimension { vert: Modulus::new(vert), horiz: Modulus::new(horiz) }
    }

    pub fn new_from_shape(sh: &[usize]) -> Self {
        WrapDimension::new(sh[0] as i32, sh[1] as i32)
    }

    pub fn compute_index(&self, coord: &Coord) -> (usize, usize) {
        (
            self.vert.apply(coord.y) as usize,
            self.horiz.apply(coord.x) as usize,
        )
    }
}

pub struct WrapCanvas<T> {
    array: Array2<T>,
    dims: WrapDimension
}

fn shape(height: u32, width: u32) -> (usize, usize) {
    (height as usize, width as usize)
}

impl<T> WrapCanvas<T> {


    pub fn height(&self) -> usize {
        self.array.shape()[0]
    }

    pub fn width(&self) -> usize {
        self.array.shape()[1]
    }

    pub fn from_fn<F: FnMut(i32, i32) -> T>(height: u32, width: u32, mut f: F) -> Self {
        let g = move |t: (usize, usize)| f(t.0 as i32, t.1 as i32);
        Array2::from_shape_fn((height as usize, width as usize), g).into()
    }
}

impl<T: Copy> WrapCanvas<T> {
    pub unsafe fn uninitialized(height: u32, width: u32) -> Self {
        Array2::uninitialized(shape(height, width)).into()
    }
}

impl<T: Default> WrapCanvas<T> {
    pub fn new(height: u32, width: u32) -> Self {
        Array2::default(shape(height, width)).into()
    }
}

impl<T: Clone> WrapCanvas<T> {
    pub fn from_elem(height: u32, width: u32, elem: T) -> Self {
        Array2::from_elem(shape(height, width), elem).into()
    }
}

impl<T> From<Array2<T>> for WrapCanvas<T> {
    fn from(arr: Array2<T>) -> Self {
        let h = arr.shape()[0] as i32;
        let w = arr.shape()[1] as i32;
        Self {
            array: arr,
            dims: WrapDimension::new(h,w)
        }
    }
}

impl<T> Index<Coord> for WrapCanvas<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &T {
        &self.array[self.dims.compute_index(&coord)]
    }
}

impl<T> IndexMut<Coord> for WrapCanvas<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.dims.compute_index(&coord);
        &mut self.array[index]
    }
}

impl<'a, T> IntoIterator for &'a WrapCanvas<T> {
    type Item = &'a T;
    type IntoIter = <&'a Array2<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<T> Into<Array2<T>> for WrapCanvas<T> {
    fn into(self) -> Array2<T> {
        self.array
    }
}

impl<T> AsRef<Array2<T>> for WrapCanvas<T> {
    fn as_ref(&self) -> &Array2<T> {
        &self.array
    }
}

impl<T> AsMut<Array2<T>> for WrapCanvas<T> {
    fn as_mut(&mut self) -> &mut Array2<T> {
        &mut self.array
    }
}

/*
macro_rules! make_wrap {
    ($n: ident, $t: ident, $i: ty) => (
        pub struct $n<T> {
            array: $t<T>,
            width: Modulus,
            height: Modulus
        }

        impl<T> Index<(i32,i32)> for $n<T> {
            type Output = <$t<T> as Index<($i,$i)>>::Output;

            fn index(&self, idx: (i32,i32)) -> &Self::Output {
                &self.array.index((idx.0 as i32,idx.1 as i32))
            }
        }

    )
}

make_wrap!(WrapCanvas, Array2, usize);
*/
