mod hyperspace;
// mod vec;
mod bitvector;

use bitvec::prelude::BitVec;
use rand::rngs::StdRng;
use rand::SeedableRng;

use hyperspace::{Algebra, Hyperspace};

fn main() {
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    let mut hyperspace = Hyperspace::<BitVec>::new(10000);

    // What is the Dollar of Mexico?

    // let usa = &hyperspace.draw("USA",&mut rng);
    // let dollar = &hyperspace.draw("Dollar", &mut rng);

    // // Bind USA to the Dollar
    // let usa_dollar = usa.product(&dollar);

    // let mexico = &hyperspace.draw("Mexico", &mut rng);
    // let peso = &hyperspace.draw("Peso", &mut rng);

    // // Bind Mexico to the Peso
    // let mexico_peso = mexico.product(&peso);
}
