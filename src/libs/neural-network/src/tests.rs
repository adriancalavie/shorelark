use super::*;

mod created_randomly {

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn neuron_is_created_correctly() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(4, &mut rng);

        approx::assert_relative_eq!(neuron.bias, -0.6255188);

        approx::assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807,].as_ref()
        );
    }

    #[test]
    fn layer_is_created_correctly() {}

    #[test]
    fn network_is_created_correctly() {}

    mod propagate {
        use super::*;

        #[test]
        fn neuron_propagates_correctly() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            // Ensures `.max()` (our ReLU) works:
            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            // `0.5` and `1.0` chosen by a fair dice roll:
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );

            // We could've written `1.15` right away, but showing the entire
            // formula makes our intentions clearer
        }

        #[test]
        fn layer_propagates_correctly() {}

        #[test]
        fn network_propagates_correctly() {}
    }
}
