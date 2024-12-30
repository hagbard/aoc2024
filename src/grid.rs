use ndarray::Array2;
use num_traits::ToPrimitive;
use crate::point::Point;

pub type Grid = Array2<char>;

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

pub(crate) trait GridLike {
    fn at(&self, p: &Point) -> Option<char>;
}

impl GridLike for Grid {
    fn at(&self, p: &Point) -> Option<char> {
        let (x, y) = *p;
        match (x.to_usize(), y.to_usize()) {
            (Some(i), Some(j)) => self.get((i, j)).map(|c| *c),
            _ => None,
        }
    }
}