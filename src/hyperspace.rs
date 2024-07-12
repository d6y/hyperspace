use std::marker::PhantomData;

use rand::Rng;

// We store vectors in the hyperspace:
pub trait Vector: Clone {
    type Element;
    fn new<R: Rng>(vector_length: usize, rng: &mut R) -> Self;
}

// We need to define the algebraic operations on the vectors:
pub trait Algebra
where
    Self: Vector,
{
    fn distance(&self, other: &Self) -> f32;
    fn add<R: Rng>(&self, other: &Self, rng: &mut R) -> Self;
    fn product(&self, other: &Self) -> Self;
}

pub struct Hyperspace<V: Vector> {
    vector_length: usize,
    vectors: Vec<V>,
    labels: Vec<String>,
    _phantom: PhantomData<V::Element>,
}

impl<V: Vector + Algebra> Hyperspace<V> {
    pub fn new(vector_length: usize) -> Self {
        Hyperspace {
            vector_length,
            vectors: Vec::new(),
            labels: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn draw<R: Rng, S: Into<String>>(&mut self, label: S, rng: &mut R) -> V {
        let v = V::new(self.vector_length, rng);
        self.vectors.push(v.clone());
        self.labels.push(label.into());
        v
    }

    // Find the closest vector to the given vector.
    fn clean_up_index(&self, v: &V) -> usize {
        assert!(!self.vectors.is_empty());

        let first = self.vectors.first().unwrap();
        let mut closest = Closest {
            index: 0,
            distance: first.distance(v),
        };

        //dbg!(closest.index, &self.labels[0], closest.distance);

        for pos in 1..self.vectors.len() {
            let candidate = &self.vectors[pos];
            let distance = candidate.distance(v);
            // dbg!(pos, &self.labels[pos], distance);
            if distance < closest.distance {
                closest.index = pos;
                closest.distance = distance;
            }
        }

        closest.index
    }

    pub fn label_for(&self, v: &V) -> &str {
        &self.labels[self.clean_up_index(v)]
    }
}

struct Closest {
    index: usize,
    distance: f32,
}
