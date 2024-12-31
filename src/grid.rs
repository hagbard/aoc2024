use std::iter;
use itertools::Itertools;
use ndarray::Array2;
use num_traits::ToPrimitive;
use crate::point::{Dir, Point, PointLike};

// Make the iterators dynamic to avoid needing lots of fiddly structs everywhere.
pub type GridIter<'a> = Box<dyn Iterator<Item=char> + 'a>;
pub type IndexedGridIter<'a> = Box<dyn Iterator<Item=(Point, char)> + 'a>;

pub struct Grid {
    arr: Array2<char>,
}

impl Grid {
    pub fn get(&self, p: &Point) -> Option<char> {
        let (x, y) = *p;
        match (x.to_usize(), y.to_usize()) {
            (Some(i), Some(j)) => self.arr.get((i, j)).map(|c| *c),
            _ => None,
        }
    }

    pub fn iter(&self) -> GridIter {
        Box::new(self.arr.iter().map(|c| *c))
    }

    pub fn indexed_iter(&self) -> IndexedGridIter {
        Box::new(self.arr.indexed_iter().map(|(p, c)| (Point::of(&p), *c)))
    }

    pub fn walk(&self, start: Option<Point>, d: Dir) -> GridIter {
        let mut cur = start;
        Box::new(iter::from_fn(move || {
            let last = cur.and_then(|p| self.get(&p));
            cur = cur.and_then(|p| d.step(1, &p));
            last
        }))
    }

    pub fn indexed_walk(&self, start: Option<Point>, d: Dir) -> IndexedGridIter {
        let mut cur = start;
        Box::new(iter::from_fn(move || {
            let last = cur.and_then(|p| self.get(&p).map(|c| (p, c)));
            cur = cur.and_then(|p| d.step(1, &p));
            last
        }))
    }
}

pub fn parse_lines(lines: &str) -> Grid {
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
    Grid { arr: Array2::from_shape_vec((height, width), ascii).unwrap() }
}
