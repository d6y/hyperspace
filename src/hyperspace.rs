use std::marker::PhantomData;

use rand::Rng;

// We store vectors in the hyperspace:
pub trait Vector {
    type Element;
    fn new<R: Rng>(vector_length: usize, rng: &mut R) -> Self;
}

// We need to define the algebraic operations on the vectors:
pub trait Algebra
where
    Self: Vector,
{
    // fn weighting(&self, weight: <Self as Vector>::Element) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn complement(&self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
    fn distance(&self, other: &Self) -> f32;
    fn product(&self, other: &Self) -> Self;
}

pub struct Hyperspace<V: Vector> {
    vector_length: usize,
    vectors: Vec<V>,
    labels: Vec<String>,
    _phantom: PhantomData<V::Element>,
}

impl<V: Vector> Hyperspace<V> {
    pub fn new(vector_length: usize) -> Self {
        Hyperspace {
            vector_length,
            vectors: Vec::new(),
            labels: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn draw<R: Rng>(&mut self, label: &str, rng: &mut R) -> &V {
        let v = V::new(self.vector_length, rng);
        self.vectors.push(v);
        self.labels.push(label.to_string());
        self.vectors.last().unwrap()
    }
}
