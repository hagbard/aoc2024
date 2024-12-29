use ndarray::Array2;
use strum_macros::EnumIter;

pub type Grid = Array2<char>;
pub type Point = (usize, usize);

pub trait PointLike {
    fn step(&self, d: Dir, n: isize) -> Option<Point>;
}

impl PointLike for Point {
    fn step(&self, d: Dir, n: isize) -> Option<Point> {
        d.step(n, self)
    }
}

pub fn parse_lines(lines: &str) -> Array2<char> {
    let mut ascii: Vec<char> = vec![];
    let mut width: usize = 0;
    let mut height: usize = 0;
    for s in lines.split('\n') {
        let before_len = ascii.len();
        ascii.extend(s.chars().map(|c| {
            assert!(c.is_ascii());
            c
        }));
        let w = ascii.len() - before_len;
        if width == 0 { width = w; } else {
            assert_eq!(w, width, "Inconsistent width");
        }
        height += 1;
    }
    // Axis 0 is height, since that's the outermost dimension to stride over.
    assert!(width > 0 && height > 0);
    Array2::from_shape_vec((height, width), ascii).unwrap()
}

#[derive(Copy, Clone, EnumIter)]
#[repr(u16)]
pub enum Dir {
    // Encode (-1, 0, 1) for both X and Y direction as signed bytes in a single u16.
    Right = 0x0100,
    RightUp = 0x0101,
    Up = 0x0001,
    LeftUp = 0xFF01,
    Left = 0xFF00,
    LeftDown = 0xFFFF,
    Down = 0x00FF,
    RightDown = 0x01FF,
}

impl Dir {
    pub fn offset(&self) -> (isize, isize) {
        let n = *self as u16;
        (((n >> 8) & 0xFF) as i8 as isize, (n & 0xFF) as i8 as isize)
    }

    pub fn step(&self, n: isize, p: &Point) -> Option<Point> {
        let (x, y) = p;
        let (ix, iy) = self.offset();
        match (ix.checked_mul(n).and_then(|m| x.checked_add_signed(m)),
               iy.checked_mul(n).and_then(|m| y.checked_add_signed(m))) {
            (Some(next_x), Some(next_y)) => {
                Some((next_x, next_y))
            }
            _ => None,
        }
    }
}

pub struct GridIter {
    p: Option<Point>,
    d: Dir,
}

impl GridIter {
    pub fn new<'a>(p: Option<Point>, d: Dir) -> GridIter {
        GridIter { p, d }
    }
}

impl Iterator for GridIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.p;
        self.p = next.and_then(|p| self.d.step(1, &p));
        next
    }
}
