use std::fmt::Debug;
use num_traits::PrimInt;

use xy::Dir::{Down, Left, Right, Up};

use crate::util::PrimIter;
use crate::xy;

/// An orthogonal direction in the X/Y plane, where (0, 0) is considered "top left".
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    /// Increasing X.
    Right,
    /// Increasing Y.
    Down,
    /// Decreasing X.
    Left,
    /// Decreasing X.
    Up,
}

impl Dir {
    pub fn reverse(&self) -> Dir {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// An absolute point in the X/Y plane.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<I: PrimInt> {
    pub x: I,
    pub y: I,
}

#[allow(unused)]
impl<I: PrimInt> Point<I> {
    pub fn new(x: I, y: I) -> Self { Point { x, y } }

    pub fn origin() -> Self {
        Point { x: I::zero(), y: I::zero() }
    }

    pub fn offset_xy(&self, x: I, y: I) -> Point<I> {
        Point { x: self.x + x, y: self.y + y }
    }

    pub fn move_by(&self, len: I, dir: Dir) -> Point<I> {
        assert!(!len.is_zero(), "Must move by non-zero length");
        let mut x = self.x;
        let mut y = self.y;
        match dir {
            Right => x = x + len,
            Down => y = y + len,
            Left => x = x - len,
            Up => y = y - len,
        };
        Point { x, y }
    }
}

impl<I: PrimInt> From<(I, I)> for Point<I> {
    fn from(xy: (I, I)) -> Self {
        Point { x: xy.0, y: xy.1 }
    }
}

#[derive(Debug)]
pub struct Piter<I: PrimInt> {
    min_axis: PrimIter<I>,
    maj_axis: PrimIter<I>,
}

impl<I: PrimInt> Piter<I> {
    pub fn new(start: Point<I>, min_dir: Dir, min_len: I, maj_dir: Dir, maj_len: I) -> Piter<I> {
        Piter {
            min_axis: Piter::get_iter(start, min_dir, min_len),
            maj_axis: Piter::get_iter(start, maj_dir, maj_len),
        }
    }

    fn get_iter(start: Point<I>, dir: Dir, len: I) -> PrimIter<I> {
        assert!(len >= I::one());
        let offset = len - I::one();
        match dir {
            Right => PrimIter::new(start.x, start.x + offset, I::one()),
            Down => PrimIter::new(start.y, start.y + offset, I::one()),
            Left => PrimIter::new(start.x, start.x - offset, I::one()),
            Up => PrimIter::new(start.y, start.y - offset, I::one()),
        }
    }
}

impl<I: PrimInt> Iterator for Piter<I> {
    type Item = Point<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.maj_axis.get() {
            if let Some(x) = self.min_axis.next() {
                return Some(Point { x, y });
            }
            self.maj_axis.next();
            if let Some(y) = self.maj_axis.get() {
                return Some(Point { x: self.min_axis.reset_and_get(), y });
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use Dir::{Left, Right, Up};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_piter() {
        let smol: Point<u8> = Point { x: 0x1, y: 0x1 };

        let it: Piter<u8> = Piter::new(smol, Right, 0x2, Down, 0x2);
        assert_eq!(it.collect::<Vec<_>>(), vec![
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
            Point::new(2, 2)]);

        let it: Piter<u8> = Piter::new(smol, Left, 0x2, Up, 0x2);
        assert_eq!(it.collect::<Vec<_>>(), vec![
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(0, 0)]);
    }
}