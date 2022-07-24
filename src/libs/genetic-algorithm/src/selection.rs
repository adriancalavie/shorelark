use rand::prelude::SliceRandom;

use crate::*;

pub trait SelectionMethod {
    fn select<'a, I>(&mut self, population: &'a [I], rng: &mut dyn rand::RngCore) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RouletteWheelSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&mut self, population: &'a [I], rng: &mut dyn rand::RngCore) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Got an empty population")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{
        individual::{Individual, TestIndividual},
        selection::{RouletteWheelSelection, SelectionMethod},
    };

    #[test]
    fn test_selection_method() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut method = RouletteWheelSelection::new();

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram: BTreeMap<i32, _> = (0..1_000)
            .map(|_| method.select(&population, &mut rng))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => how many times this fitness has been chosen
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
