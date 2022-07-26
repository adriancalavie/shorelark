use crate::*;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn to_chromosome(&self) -> &Chromosome;
    fn from_chromosome(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    /// For tests that require access to chromosome
    WithChromosome { chromosome: Chromosome },

    /// For tests that don't require access to chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn from_chromosome(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn to_chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => {
                chromosome.iter().sum()

                // ^ the simplest fitness function ever - we're just
                // summing all the genes together
            }

            Self::WithFitness { fitness } => *fitness,
        }
    }
}
