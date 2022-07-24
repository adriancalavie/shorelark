use rand::{Rng, RngCore};

use crate::*;

pub trait MutationMethod {
    fn mutate(&self, child: &mut Chromosome, rng: &mut dyn RngCore);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, child: &mut Chromosome, rng: &mut dyn RngCore) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::{GaussianMutation, MutationMethod};

    fn get_actual_child(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff).mutate(&mut child, &mut rng);

        child.into_iter().collect()
    }
    mod given_zero_chance {
        fn get_actual_child(coeff: f32) -> Vec<f32> {
            super::get_actual_child(0.0, coeff)
        }
        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = get_actual_child(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = get_actual_child(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }
    }

    mod given_fifty_fifty_chance {
        fn get_actual_child(coeff: f32) -> Vec<f32> {
            super::get_actual_child(0.5, coeff)
        }
        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = get_actual_child(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = get_actual_child(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }
    }

    mod given_max_chance {
        fn get_actual_child(coeff: f32) -> Vec<f32> {
            super::get_actual_child(1.0, coeff)
        }
        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = get_actual_child(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = get_actual_child(0.5);

                let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }
    }
}
