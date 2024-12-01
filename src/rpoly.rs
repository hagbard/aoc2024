use std::iter::Chain;
use std::slice::Iter;
use std::vec;

use itertools;
use itertools::{Itertools, TupleWindows};

use crate::xy::{Dir, Orientation, Point};
use crate::xy::Dir::{Down, Left, Right, Up};
use crate::xy::Orientation::{Horizontal, Vertical};

/// A "rectangular" polygon consisting of only orthogonal horizontal and vertical edges.
///
/// An `RPoly` instance is "closed" if the final edge, implicitly formed between the last and first
/// added points is itself orthogonal. This implies that the minimum closed instance must have at
/// least 4 points/edges.
///
/// Most operations can only be carried out on closed `RPoly` instances.
#[derive(Debug)]
pub struct RPoly {
    poly: Vec<Point<i32>>,
}

// NOTE: The explicit <'a> lifetime exists to bind the lifetime of 'poly' to the devired structures
// using 'Edge'. Only methods which use Edge instances need it, and RPoly itself owns all its data
// directly, avoiding the need for lifetimes to be specified. Essentially 'Edge' is an inner class
// of RPoly and connot outlive it (but it's not directly held by an instance, and only created in
// methods which need it).
#[allow(unused)]
impl<'a> RPoly {
    /// Creates an empty `RPoly` with no points in it.
    pub fn new() -> RPoly { RPoly { poly: vec![] } }

    /// Adds a copy of the given point to the current polygon.
    pub fn add_point(&mut self, p: &Point<i32>) {
        self.poly.push(*p);
    }

    /// Adds the given `(x, y)` point to the current polygon.
    pub fn add_xy(&mut self, x: i32, y: i32) {
        self.poly.push(Point { x, y });
    }

    /// Adds a new point to the polygon relative to the last added point or, if the polygon is
    /// empty, the origin.
    pub fn add_relative(&mut self, dir: Dir, len: i32) {
        self.poly.push(self.poly.last().unwrap_or(&Point::origin()).move_by(len, dir));
    }

    /// Returns the perimeter of a closed RPoly.
    pub fn get_perimeter(&self) -> i64 {
        RPoly::sum_edge_lengths(&self.get_orthogonal_edges())
    }

    /// Returns the "aligned" area of a closed RPoly. Aligned area is the area covered by the
    /// polygon in the X/Y plane if the points of the polygon are aligned to the X/Y coordinates.
    ///
    /// Aligned area is the natural area of a polygon if edges have no width.
    pub fn get_area(&self) -> i64 {
        RPoly::get_edge_aligned_area(&self.get_orthogonal_edges())
    }

    /// Returns the "external" area of a closed `RPoly`, which includes the contribution of edges
    /// assumed to have a total width of 1 unit.
    ///
    /// This can be thought of as the area of the polygon formed by displacing every point by
    /// `(0.5, 0.5)` and then expanding the area to align with the coordinate grid of the X/Y
    /// plane.
    ///
    /// An example showing how a point `X = (1, 1)` is offset to the center of the grid square and
    /// then the area is expanded "outwards". This result is the same as
    /// `get_internal_area() + get_perimeter()`.
    ///
    /// ```
    /// 3 +
    ///   |   | :   Polygon point (1, 1) displaced by "half a grid square".
    /// 2 +   | : /
    ///   |   | X....
    /// 1 +   .___.___._ ... Edge of external area.
    ///   |
    /// 0 |___.___.___.___
    ///   0   1   2   3
    /// ```
    pub fn get_external_area(&'a self) -> i64 {
        // We must capture the explicit lifetime for 'self' because it also applies to 'edges'.
        let edges: Vec<Edge<'a>> = self.get_orthogonal_edges();
        let perimeter = RPoly::sum_edge_lengths(&edges);
        let aligned_area = RPoly::get_edge_aligned_area(&edges);
        aligned_area + 1 + (perimeter / 2)
    }

    /// Returns the "internal" area of a closed `RPoly`, which excludes the contribution of edges
    /// assumed to have a total width of 1 unit.
    ///
    /// This can be thought of as the area of the polygon formed by displacing every point by
    /// `(0.5, 0.5)` and then contracting the area to align with the coordinate grid of the X/Y
    /// plane.
    ///
    /// An example showing how a point `X = (1, 1)` is offset to the center of the grid square and
    /// then the area is contracted "inwards". This result is the same as
    /// `get_external_area() - get_perimeter()`.
    ///
    /// ```
    /// 3 +   .
    ///   |     : |
    /// 2 +   . : |___._ ... Edge of internal area.
    ///   |     X.... Polygon point (1, 1) displaced by "half a grid square".
    /// 1 +   .   .   .
    ///   |
    /// 0 |___.___.___.___
    ///   0   1   2   3
    /// ```
    pub fn get_internal_area(&'a self) -> i64 {
        // We must capture the explicit lifetime for 'self' because it also applies to 'edges'.
        let edges: Vec<Edge<'a>> = self.get_orthogonal_edges();
        let perimeter = RPoly::sum_edge_lengths(&edges);
        let aligned_area = RPoly::get_edge_aligned_area(&edges);
        aligned_area + 1 - (perimeter / 2)
    }

    fn get_orthogonal_edges(&'a self) -> Vec<Edge<'a>> {
        // We must capture the explicit lifetime for 'self' because it also applies to 'edges'.
        let len = self.poly.len();
        assert!(len >= 4, "Not enough points!");
        let edges: Vec<Edge<'a>> =
            (0..len).map(|i| Edge { start: &self.poly[i], end: &self.poly[(i + 1) % len] }).collect();
        assert!(to_cyclic_pairs(&edges).all(|(a, b)| a.orientation() != b.orientation()));
        edges
    }

    fn sum_edge_lengths(edges: &Vec<Edge>) -> i64 {
        // Here there's no need to care about the lifetime of 'edges' since the result value is
        // not bound to it.
        edges.iter().map(|e| e.length() as i64).sum()
    }

    fn get_edge_aligned_area(edges: &'a Vec<Edge<'a>>) -> i64 {
        // List of vertical edges in "Polygon order".
        //
        // Here, 'vedges' is borrowing the edge references for 'edges' and must therefore reflect
        // the lifetime explicitly. Note the use of "&&'a", which indicates an unbound reference
        // (generated during iteration) to the lifetime bound reference of the source vector.
        //
        // Once we have 'vedges', we never need to discuss its lifecycle again because all the
        // results it produces are not lifetime bound.
        //
        // If 'Edge' were copyable we could make 'vedges: Vec<Edge<'a>>' but would still need the
        // lifetime specified for 'Edge'
        let vedges: Vec<&'a Edge<'a>> = edges.iter().filter(|e: &&'a Edge<'a>| e.orientation() == Vertical).collect();

        // Y-coordinates at which changes to followers must be made.
        //
        // The assoicated index is that of the edge which *ends* at the Y position. In general:
        //     cur = vedges[i], nxt = vedges[j]
        // where j = 1+/-1 depending on polygon/edge orientation.
        //
        // Changes are one of:
        // * cur==down, nxt==up: Add cur/nxt as edge followers
        // * cur==up, nxt==down: Remove cur/nxt as edge followers
        // * cur==up, nxt==up: Replace cur with nxt
        // * cur==down, nxt==down: Replace nxt with cur
        let y_values: Vec<(i32, usize)> =
            vedges.iter().enumerate().map(|(vi, ve)| (ve.end().y, vi)).sorted().collect();

        // (x, idx)
        let mut active: Vec<(i32, usize)> = vec![];
        let mut y_prv: i32 = y_values[0].0;
        let mut y_idx: usize = 0;
        let mut total_area: i64 = 0;
        'main: loop {
            let (mut y_cur, mut vi) = y_values[y_idx];
            assert!(y_cur >= y_prv);
            while y_cur == y_prv {
                let va = &vedges[vi];
                assert_eq!(insert_or_remove(&mut active, (va.end().x, vi)), va.direction() == Up);
                let (vb, vj) = next(&vedges, vi);
                assert_eq!(insert_or_remove(&mut active, (vb.end().x, vj)), vb.direction() == Down);
                y_idx += 1;
                if y_idx == y_values.len() { break 'main; }
                (y_cur, vi) = y_values[y_idx];
            }
            let width: i32 = active.iter()
                .sorted().tuples().map(|(&lhs, &rhs)| rhs.0 - lhs.0).sum();
            total_area += ((y_cur - y_prv) as i64) * (width as i64);
            y_prv = y_cur;
        }
        assert!(active.is_empty());
        total_area
    }
}

// Demonstrates usage of lifetimes for what is effectively an inner struct of RPoly.
//
// An edge directly references the points within an RPoly's vector. This means the edge must have
// an explicit named lifetime so to be associated with a specific RPoly instance.
#[derive(Debug)]
struct Edge<'a> {
    start: &'a Point<i32>,
    end: &'a Point<i32>,
}

// https://stackoverflow.com/questions/39355984/what-does-the-first-explicit-lifetime-specifier-on-an-impl-mean
//
// When only "&self" is given to a method, it is still assumed to have the lifecycle of the struct,
// and any returned reference is also assumed to have the lifecycle at least as long as the struct.
//
// This means (because the struct and field lifetimes are the same) the code itself needs no
// explicit mentions of lifetimes.
//
//    ,,-- This indicates an explicit lifetime given by the compiler.
//    vv       vv-- Which is associated to the lifetime of the struct and, in turn, its contents.
impl<'a> Edge<'a> {
    fn start(&self) -> &Point<i32> {
        &self.start
    }

    fn end(&self) -> &Point<i32> {
        &self.end
    }

    fn width(&self) -> i32 {
        (self.end().x - self.start().x).abs()
    }

    fn height(&self) -> i32 {
        (self.end().y - self.start().y).abs()
    }

    fn orientation(&self) -> Orientation {
        if self.width() != 0 { Horizontal } else { Vertical }
    }

    fn direction(&self) -> Dir {
        let start = self.start();
        let end = self.end();
        let h_offset = end.x - start.x;
        let v_offset = end.y - start.y;
        if h_offset != 0 {
            if h_offset > 0 { Right } else { Left }
        } else {
            if v_offset > 0 { Down } else { Up }
        }
    }

    fn length(&self) -> i32 {
        let w = self.width();
        if w != 0 { w } else { self.height() }
    }
}

fn next<I>(vec: &Vec<I>, mut i: usize) -> (&I, usize) {
    i = if i < vec.len() - 1 { i + 1 } else { 0 };
    return (&vec[i], i);
}

fn to_cyclic_pairs<I>(items: &Vec<I>) -> TupleWindows<Chain<Iter<I>, Iter<I>>, (&I, &I)> {
    items[items.len() - 1..].iter().chain(items.iter()).tuple_windows::<(&I, &I)>()
}

fn insert_or_remove(active: &mut Vec<(i32, usize)>, pos: (i32, usize)) -> bool {
    match active.binary_search(&pos) {
        Err(i) => {
            active.insert(i, pos);
            true
        }
        Ok(i) => {
            active.remove(i);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_poly() {
        /*
         *     7--6   3--2
         *     |  |   |  |
         * Y   |  5---4  |
         * ^   |         |
         * >X  0---------1
         */
        let mut poly = RPoly::new();
        poly.add_relative(Right, 10);
        poly.add_relative(Down, 10);
        poly.add_relative(Left, 3);
        poly.add_relative(Up, 5);
        poly.add_relative(Left, 4);
        poly.add_relative(Down, 5);
        poly.add_relative(Left, 3);
        poly.add_relative(Up, 10);
        assert_eq!(poly.get_perimeter(), 50);
        assert_eq!(poly.get_area(), 80);
        assert_eq!(poly.get_external_area(), 106);
        assert_eq!(poly.get_internal_area(), 56);
    }
}