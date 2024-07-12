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

    fn add<R: Rng>(&self, other: &Self, rng: &mut R) -> Self {
        assert!(self.len() == other.len());

        let len = self.len();

        let mut result = bitvec![0; len];

        for i in 0..len {
            result.set(
                i,
                match (self[i], other[i]) {
                    (true, true) => true,
                    (false, false) => false,
                    (true, false) | (false, true) => rng.gen(),
                },
            );
        }

        result
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
        let a = bitvec![0, 1, 0, 1];
        let b = bitvec![0, 0, 1, 1];

        let sum = a.add(&b, &mut rand::thread_rng());

        // There is a random choice where there's an equal
        // number of 1s and 0s

        assert!(sum[0] == false);
        assert!(sum[3] == true);
    }

    #[test]
    fn product() {
        let a = bitvec![0, 1, 0, 1];
        let b = bitvec![1, 1, 0, 0];
        let expected = bitvec![1, 0, 0, 1];
        assert_eq!(a.product(&b), expected);
    }
}
