use crate::hyperspace::{Algebra, Vector};
use bitvec::prelude::*;
use rand::Rng;

impl Vector for BitVec {
    type Element = i8;

    fn new<R: Rng>(vector_length: usize, rng: &mut R) -> Self {
        let mut bv = BitVec::with_capacity(vector_length);
        for _ in 0..vector_length {
            bv.push(rng.gen::<bool>());
        }
        bv
    }
}

impl Algebra for BitVec {
    fn distance(&self, other: &Self) -> f32 {
        assert!(self.len() == other.len());
        let mut xored = self.clone();
        xored ^= other;
        xored.count_ones() as f32 / self.len() as f32
    }

    fn add(&self, other: &Self) -> Self {
        assert!(self.len() == other.len());
        let mut sum = self.clone();
        sum |= other;
        sum
    }

    fn complement(&self) -> Self {
        use std::ops::Not;
        self.clone().not()
    }

    fn subtract(&self, other: &Self) -> Self {
        assert!(self.len() == other.len());
        self.add(&other.complement())
    }

    fn product(&self, other: &Self) -> Self {
        assert!(self.len() == other.len());
        let mut xored = self.clone();
        xored ^= other;
        xored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_identical() {
        let example = bitvec![0, 1, 0, 1, 0, 1, 0, 1];
        let dist = example.distance(&example);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn similarity_opposite() {
        let top_left = bitvec![0, 1];
        let bottom_right = bitvec![1, 0];
        let dist = top_left.distance(&bottom_right);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn add() {
        let a = bitvec![1, 0, 1, 0];
        let b = bitvec![0, 1, 0, 0];
        let expected = bitvec![1, 1, 1, 0];

        let sum = a.add(&b);
        assert_eq!(sum, expected);
    }

    #[test]
    fn complement() {
        let input = bitvec![1, 0];
        let expected = bitvec![0, 1];
        assert_eq!(input.complement(), expected);
    }

    #[test]
    fn subtract() {
        let a = bitvec![1, 0, 1, 0];
        let b = bitvec![1, 1, 0, 0];

        let e = bitvec![1, 0, 1, 1];

        let r = a.subtract(&b);
        assert_eq!(r, e);
    }

    #[test]
    fn product() {
        let a = bitvec![0, 1, 0, 1];
        let b = bitvec![1, 1, 0, 0];
        let expected = bitvec![1, 0, 0, 1];
        assert_eq!(a.product(&b), expected);
    }
}
