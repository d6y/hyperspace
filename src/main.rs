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

    let mut hyperspace = Hyperspace::<BitVec>::new(10_000);

    // Random vectors for concepts.
    // We store them in the hyperspace and given them a text label.
    let country = &hyperspace.draw("Country", &mut rng);
    let currency = &hyperspace.draw("Currency", &mut rng);

    // Random vectors for four values:
    let usa = &hyperspace.draw("USA", &mut rng);
    let dollar = &hyperspace.draw("Dollar", &mut rng);
    let mexico = &hyperspace.draw("Mexico", &mut rng);
    let peso = &hyperspace.draw("Peso", &mut rng);

    // Facts
    // NB these happen not to be stored in the hyperspace.

    // Binding concepts to values:
    let usa_isa_country = usa.product(country);
    let mexico_isa_country = mexico.product(country);
    let dollar_isa_currency = dollar.product(currency);
    let peso_isa_currency = peso.product(currency);

    // More facts, associating a country to a currency
    let usa_currency = usa_isa_country.add(&dollar_isa_currency, &mut rng);
    let mexico_currency = mexico_isa_country.add(&peso_isa_currency, &mut rng);

    // Retrieval:

    // Let's check we can look up the dollar label from the vector:
    assert_eq!(&"Dollar", &hyperspace.label_for(dollar));

    // Let's unpick the currency field from the usa_currency vector:
    let us_currency_query = currency.product(&usa_currency);
    assert_eq!(&"Dollar", &hyperspace.label_for(&us_currency_query));

    // What role does the dollar play in the US?
    let dollar_query = dollar.product(&usa_currency);
    assert_eq!(&"Currency", &hyperspace.label_for(&dollar_query));

    // And what's the "Dollar" of Mexico?
    let mexico_dollar_query = (dollar.product(&usa_currency)).product(&mexico_currency);
    // Closest vector is:
    assert_eq!(&"Peso", &hyperspace.label_for(&mexico_dollar_query));

    // What is the USA of the Peso?
    let usa_of_peso_query = (usa.product(&usa_currency)).product(&mexico_currency);
    assert_eq!(&"Mexico", &hyperspace.label_for(&usa_of_peso_query));
}
