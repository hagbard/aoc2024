use std::collections::HashMap;
use std::hash::Hash;
use num_traits::PrimInt;
use strum_macros::{Display, EnumIter};

#[derive(Debug)]
pub struct PrimIter<I: PrimInt> {
    start: I,
    end: I,
    step: I,
    cur: Option<I>,
}

impl<I: PrimInt> PrimIter<I> {
    pub fn new(start: I, end: I, step: I) -> PrimIter<I> {
        PrimIter { start, end, step, cur: Some(start) }
    }

    pub fn get(&self) -> Option<I> { self.cur }

    pub fn reset_and_get(&mut self) -> I {
        self.cur = Some(self.start);
        self.next();
        self.start
    }
}

impl<I: PrimInt> Iterator for PrimIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.cur;
        if let Some(n) = value {
            if self.end >= self.start {
                self.cur = if self.end - n >= self.step {
                    Some(n + self.step)
                } else { None };
            } else {
                self.cur = if n - self.end >= self.step {
                    Some(n - self.step)
                } else { None }
            }
        }
        return value;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_iter() {
        let it: PrimIter<usize> = PrimIter::new(20, 10, 2);
        assert_eq!(it.collect::<Vec<_>>(), [20, 18, 16, 14, 12, 10]);
    }
}

#[derive(Display, EnumIter, Debug)]
pub enum Digits {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
}
