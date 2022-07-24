use std::iter::once;

use rand::Rng;
#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}
#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}
#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}
#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    pub fn random(layers: &[LayerTopology], rng: &mut dyn rand::RngCore) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons, rng))
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> Vec<f32> {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
            .collect()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }

    #[cfg(test)]
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        assert!(layers.len() > 1);

        Self { layers }
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub(crate) fn random(
        input_neurons: usize,
        output_neurons: usize,
        rng: &mut dyn rand::RngCore,
    ) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons, rng))
            .collect();

        Self { neurons }
    }

    pub(crate) fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Layer {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    #[cfg(test)]
    pub(crate) fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    fn random(output_size: usize, rng: &mut dyn rand::RngCore) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }

    #[cfg(test)]
    pub(crate) fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests;
