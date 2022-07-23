use rand::RngCore;

use crate::{
    crossover::CrossoverMethod, individual::Individual, mutation::MutationMethod,
    selection::SelectionMethod,
};

pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn evolve<I>(&mut self, population: &[I], rng: &mut dyn RngCore) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                let parent_a = self
                    .selection_method
                    .select(population, rng)
                    .to_chromosome();
                let parent_b = self
                    .selection_method
                    .select(population, rng)
                    .to_chromosome();

                let mut child = self.crossover_method.crossover(parent_a, parent_b, rng);

                self.mutation_method.mutate(&mut child, rng);

                I::from_chromosome(child)
            })
            .collect()
    }

    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        crossover::UniformCrossover, individual::TestIndividual, mutation::GaussianMutation,
        selection::RouletteWheelSelection,
    };

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn create_individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::from_chromosome(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let mut ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            create_individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            create_individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            create_individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            create_individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particular reason for a number of 10 - this test would
        // be fine for 5, 20 or even 1000 generations; the only thing
        // that'd change is the *magnitude* of difference between
        // initial and output population.
        for _ in 0..10 {
            population = ga.evolve(&population, &mut rng);
        }

        let expected_population = vec![
            create_individual(&[0.447_694_9, 2.0648358, 4.3058133]),
            create_individual(&[1.212_686_7, 1.5538777, 2.886_911]),
            create_individual(&[1.061_767_8, 2.265_739, 4.428_764]),
            create_individual(&[0.95909685, 2.4618788, 4.024_733]),
        ];

        assert_eq!(population, expected_population);
    }
}
