use std::fmt::Debug;

use ndarray::{Array2, ArrayView1, Axis, Ix1};
use ndarray::iter::Lanes;

use crate::xy::{Dir, Piter, Point};

#[derive(Debug)]
pub struct AGrid {
    pub grid: Array2<char>,
}

#[allow(unused)]
impl AGrid {
    pub fn get(&self, p: &Point<usize>) -> Option<char> {
        self.grid.get((p.y, p.x)).map(|&c| c)
    }

    pub fn get_xy(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get((y, x)).map(|&c| c)
    }

    pub fn at(&self, pos: &Point<usize>) -> Option<GPoint> {
        self.get(pos).map(|chr| GPoint { pos: *pos, chr })
    }

    pub fn at_xy(&self, x: usize, y: usize) -> Option<GPoint> {
        self.get_xy(x, y).map(|chr| GPoint { pos: Point { x, y }, chr })
    }

    fn it(&self, start: Point<usize>, min_dir: Dir, min_len: usize, maj_dir: Dir, maj_len: usize) -> impl Iterator<Item=GPoint> + Debug + '_ {
        Giter { grid: &self, it: Piter::new(start, min_dir, min_len, maj_dir, maj_len) }
    }

    pub fn all_points(&self) -> impl Iterator<Item=GPoint> + Debug + '_ {
        self.it(Point::origin(), Dir::Right, self.width(), Dir::Down, self.height())
    }

    pub fn points_from(&self, p: &Point<usize>, dirn: Dir) -> impl Iterator<Item=GPoint> + Debug + '_ {
        self.check_valid_point(p);
        match dirn {
            Dir::Right => self.it(*p, Dir::Right, self.width() - p.x, Dir::Up, 1),
            Dir::Down => self.it(*p, Dir::Right, 1, Dir::Down, self.height() - p.y),
            Dir::Left => self.it(*p, Dir::Left, p.x + 1, Dir::Up, 1),
            Dir::Up => self.it(*p, Dir::Right, 1, Dir::Up, p.y + 1),
        }
    }

    pub fn points_after(&self, p: &Point<usize>, dirn: Dir) -> impl Iterator<Item=GPoint> + Debug + '_ {
        self.points_from(p, dirn).skip(1)
    }

    pub fn is_valid(&self, p: &Point<usize>) -> bool {
        p.x < self.width() && p.y < self.height()
    }

    fn check_valid_point(&self, p: &Point<usize>) {
        assert!(p.x < self.width() && p.y < self.height());
    }

    pub fn width(&self) -> usize {
        self.grid.len_of(Axis(1))
    }

    pub fn height(&self) -> usize {
        self.grid.len_of(Axis(0))
    }

    pub fn row(&self, y: usize) -> ArrayView1<'_, char> {
        self.grid.row(y)
    }

    pub fn rows(&self) -> Lanes<'_, char, Ix1> {
        self.grid.rows()
    }

    pub fn col(&self, x: usize) -> ArrayView1<'_, char> {
        self.grid.column(x)
    }

    pub fn cols(&self) -> Lanes<'_, char, Ix1> {
        self.grid.columns()
    }

    pub fn from_lines(s: &str) -> AGrid {
        s.split('\n').collect()
    }
}

impl<'a> FromIterator<&'a str> for AGrid {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        let mut ascii: Vec<char> = vec![];
        let mut width: usize = 0;
        let mut height: usize = 0;
        for s in iter {
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
        let grid = Array2::from_shape_vec((height, width), ascii).unwrap();
        AGrid { grid }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GPoint {
    pub pos: Point<usize>,
    pub chr: char,
}

impl From<(Point<usize>, char)> for GPoint {
    fn from(v: (Point<usize>, char)) -> Self {
        GPoint { pos: v.0, chr: v.1 }
    }
}

#[derive(Debug)]
struct Giter<'a> {
    grid: &'a AGrid,
    it: Piter<usize>,
}

impl<'a> Iterator for Giter<'a> {
    type Item = GPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(move |p| GPoint { pos: p, chr: self.grid.get(&p).unwrap() })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use ndarray::array;

    use Dir::Right;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_str() {
        let s = indoc! {"abcdef
           ghijkl
           mnopqr
           stuvwx"};
        let g = AGrid::from_lines(s);
        assert_eq!(g.row(2), array!['m','n','o','p','q','r']);
        assert_eq!(g.col(3), array!['d','j','p','v']);
    }

    #[test]
    fn test_iter() {
        let s = indoc! {"abcdef
           ghijkl
           mnopqr
           stuvwx"};
        let g = AGrid::from_lines(s);

        assert_eq!(g.points_from(&Point::new(5, 2), Right).map(|p| p.chr).collect::<Vec<_>>(), vec!['r', 'q', 'p', 'o', 'n', 'm']);
    }
}
