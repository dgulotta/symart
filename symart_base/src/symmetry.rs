use alga::general::Ring;
use na::{Matrix2, Point2, Scalar, Vector2};
use num_traits::{one, zero};
use strum_macros::{Display, EnumCount, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    EnumCount,
    EnumString,
    IntoStaticStr,
)]
pub enum SymmetryGroup {
    CM,
    CMM,
    P1,
    P2,
    P3,
    P31M,
    P3M1,
    P4,
    P4G,
    P4M,
    P6,
    P6M,
    PG,
    PGG,
    PM,
    PMG,
    PMM,
}

impl SymmetryGroup {
    pub fn num_symmetries(self) -> usize {
        use self::SymmetryGroup::*;
        match self {
            P1 => 1,
            P2 | CM | PG | PM => 2,
            P3 => 3,
            CMM | P4 | PGG | PMG | PMM => 4,
            P31M | P3M1 | P6 => 6,
            P4G | P4M => 8,
            P6M => 12,
        }
    }
}

pub struct Transformation<T: Scalar> {
    matrix: Matrix2<T>,
    offset: Vector2<T>,
}

impl<T: Scalar + Ring> Transformation<T> {
    pub fn apply(&self, pt: &Point2<T>) -> Point2<T> {
        Point2::from(self.matrix * pt.coords + self.offset)
    }

    pub fn new(xx: T, xy: T, x1: T, yx: T, yy: T, y1: T) -> Self {
        Self {
            matrix: Matrix2::new(xx, xy, yx, yy),
            offset: Vector2::new(x1, y1),
        }
    }

    pub fn new_origin(xx: T, xy: T, yx: T, yy: T) -> Self {
        Self {
            matrix: Matrix2::new(xx, xy, yx, yy),
            offset: zero(),
        }
    }

    pub fn id() -> Self {
        Self {
            matrix: Matrix2::identity(),
            offset: zero(),
        }
    }

    pub fn rot60() -> Self {
        Self::new_origin(zero(), -T::one(), one(), one())
    }

    pub fn rot90() -> Self {
        Self::new_origin(zero(), -T::one(), one(), zero())
    }

    pub fn rot120() -> Self {
        Self::new_origin(-T::one(), -T::one(), one(), zero())
    }

    pub fn rot180() -> Self {
        Self::new_origin(-T::one(), zero(), zero(), -T::one())
    }

    pub fn rot240() -> Self {
        Self::new_origin(zero(), one(), -T::one(), -T::one())
    }

    pub fn rot270() -> Self {
        Self::new_origin(zero(), one(), -T::one(), zero())
    }

    pub fn rot300() -> Self {
        Self::new_origin(one(), one(), -T::one(), zero())
    }

    pub fn flip_h() -> Self {
        Self::new_origin(-T::one(), zero(), zero(), one())
    }

    pub fn flip_v() -> Self {
        Self::new_origin(one(), zero(), zero(), -T::one())
    }

    pub fn flip_d1() -> Self {
        Self::new_origin(zero(), one(), one(), zero())
    }

    pub fn flip_d2() -> Self {
        Self::new_origin(zero(), -T::one(), -T::one(), zero())
    }

    pub fn flip_d3() -> Self {
        Self::new_origin(-T::one(), -T::one(), zero(), one())
    }

    pub fn flip_d4() -> Self {
        Self::new_origin(one(), one(), zero(), -T::one())
    }

    pub fn flip_d5() -> Self {
        Self::new_origin(one(), zero(), -T::one(), -T::one())
    }

    pub fn flip_d6() -> Self {
        Self::new_origin(-T::one(), zero(), one(), one())
    }

    pub fn flip_d1_off(offset: T) -> Self {
        Self::new(zero(), one(), offset, one(), zero(), offset)
    }

    pub fn flip_d2_off(offset: T) -> Self {
        Self::new(zero(), -T::one(), offset, -T::one(), zero(), offset)
    }

    pub fn glide_x(glide: T, offset: T) -> Self {
        Self::new(one(), zero(), glide, zero(), -T::one(), offset)
    }

    pub fn glide_y(glide: T, offset: T) -> Self {
        Self::new(-T::one(), zero(), offset, zero(), one(), glide)
    }
}

type Tr<T> = Transformation<T>;

pub fn transformations<T: Scalar + Ring>(sg: SymmetryGroup, hsz: T) -> Vec<Transformation<T>> {
    use self::SymmetryGroup::*;
    match sg {
        CM => vec![Tr::id(), Tr::flip_d1()],
        CMM => vec![Tr::id(), Tr::rot180(), Tr::flip_d1(), Tr::flip_d2()],
        P1 => vec![Tr::id()],
        P2 => vec![Tr::id(), Tr::rot180()],
        P3 => vec![Tr::id(), Tr::rot120(), Tr::rot240()],
        P31M => vec![
            Tr::id(),
            Tr::rot120(),
            Tr::rot240(),
            Tr::flip_d2(),
            Tr::flip_d4(),
            Tr::flip_d6(),
        ],
        P3M1 => vec![
            Tr::id(),
            Tr::rot120(),
            Tr::rot240(),
            Tr::flip_d1(),
            Tr::flip_d3(),
            Tr::flip_d5(),
        ],
        P4 => vec![Tr::id(), Tr::rot90(), Tr::rot180(), Tr::rot270()],
        P4G => vec![
            Tr::id(),
            Tr::rot90(),
            Tr::rot180(),
            Tr::rot270(),
            Tr::glide_x(hsz, hsz),
            Tr::glide_y(hsz, hsz),
            Tr::flip_d1_off(hsz),
            Tr::flip_d2_off(hsz),
        ],
        P4M => vec![
            Tr::id(),
            Tr::rot90(),
            Tr::rot180(),
            Tr::rot270(),
            Tr::flip_v(),
            Tr::flip_h(),
            Tr::flip_d1(),
            Tr::flip_d2(),
        ],
        P6 => vec![
            Tr::id(),
            Tr::rot60(),
            Tr::rot120(),
            Tr::rot180(),
            Tr::rot240(),
            Tr::rot300(),
        ],
        P6M => vec![
            Tr::id(),
            Tr::rot60(),
            Tr::rot120(),
            Tr::rot180(),
            Tr::rot240(),
            Tr::rot300(),
            Tr::flip_d1(),
            Tr::flip_d2(),
            Tr::flip_d3(),
            Tr::flip_d4(),
            Tr::flip_d5(),
            Tr::flip_d6(),
        ],
        PG => vec![Tr::id(), Tr::glide_x(hsz, hsz)],
        PGG => vec![
            Tr::id(),
            Tr::rot180(),
            Tr::glide_x(hsz, hsz),
            Tr::glide_y(hsz, hsz),
        ],
        PM => vec![Tr::id(), Tr::flip_h()],
        PMG => vec![
            Tr::id(),
            Tr::rot180(),
            Tr::glide_x(hsz, zero()),
            Tr::glide_y(zero(), hsz),
        ],
        PMM => vec![Tr::id(), Tr::rot180(), Tr::flip_v(), Tr::flip_h()],
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridNorm {
    Square,
    Hexagonal,
}

pub fn norm_orthogonal<T: Scalar + Ring>(v: &Vector2<T>) -> T {
    v.x * v.x + v.y * v.y
}

pub fn norm_hexagonal<T: Scalar + Ring>(v: &Vector2<T>) -> T {
    v.x * v.x + v.x * v.y + v.y * v.y
}

impl GridNorm {
    pub fn from_symmetry(g: SymmetryGroup) -> GridNorm {
        use self::SymmetryGroup::*;
        match g {
            P3 | P31M | P3M1 | P6 | P6M => GridNorm::Hexagonal,
            _ => GridNorm::Square,
        }
    }

    pub fn norm<T: Scalar + Ring>(self, v: &Vector2<T>) -> T {
        match self {
            GridNorm::Square => norm_orthogonal(v),
            GridNorm::Hexagonal => norm_hexagonal(v),
        }
    }
}
