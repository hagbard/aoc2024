use std::array::IntoIter;
use strum_macros::EnumIter;

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
    pub fn cardinals() -> IntoIter<Dir, 4> {
        // Slightly cheeky to return an Iterator struct defined elsewhere, but this is all in
        // service of making the business logic read more cleanly. E.g.:
        //   Dir::cardinals().map(|&d| ...)
        // rather than:
        //   Dir::cardinals().into_iter().map(|&d| ...)
        //   Dir::cardinals().iter().map(|&&d| ...)
        [Dir::Right, Dir::Up, Dir::Left, Dir::Down].into_iter()
    }

    pub fn diagonals() -> IntoIter<Dir, 4> {
        [Dir::RightUp, Dir::LeftUp, Dir::LeftDown, Dir::RightDown].into_iter()
    }

    pub fn offset(&self) -> Point {
        let n = *self as u16;
        (((n >> 8) & 0xFF) as i8 as isize, (n & 0xFF) as i8 as isize)
    }

    pub fn step(&self, n: isize, p: &Point) -> Option<Point> {
        let (x, y) = p;
        let (ix, iy) = self.offset();
        match (ix.checked_mul(n).and_then(|m| x.checked_add(m)),
               iy.checked_mul(n).and_then(|m| y.checked_add(m))) {
            (Some(next_x), Some(next_y)) => {
                Some((next_x, next_y))
            }
            _ => None,
        }
    }
}

pub type Point = (isize, isize);

pub trait PointLike {
    fn step(&self, d: Dir, n: isize) -> Option<Point>;

    fn of(p: &(usize, usize)) -> Point;
}

impl PointLike for Point {
    fn step(&self, d: Dir, n: isize) -> Option<Point> {
        d.step(n, self)
    }

    fn of(idx: &(usize, usize)) -> Point {
        let &(x, y) = idx;
        (x as isize, y as isize)
    }
}
