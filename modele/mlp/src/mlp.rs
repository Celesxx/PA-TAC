use crate::layer::DenseLayer;
use crate::activation::{relu, sigmoid, tanh};
use crate::optimizer::{GradientDescent, MomentumGD};

pub struct MLP {
    layers: Vec<DenseLayer>,
    optimizer: GradientDescent,
}

impl MLP {
    pub fn new(sizes: &[usize], learning_rate: f32) -> Self {
        let layers = sizes
            .windows(2)
            .map(|w| DenseLayer::new(w[0], w[1]))
            .collect();
        Self {
            layers,
            optimizer: GradientDescent::new(learning_rate),
        }
    }

    pub fn forward(&self, inputs: &[f64]) -> Vec<f64> {
        self.layers.iter().fold(inputs.to_vec(), |acc, layer| layer.forward(&acc))
    }

    pub fn train(&mut self, inputs: &[f64], targets: &[f64]) {
        // Forward pass
        let mut activations = vec![inputs.to_vec()];
        for layer in &self.layers {
            let output = layer.forward(activations.last().unwrap());
            activations.push(output);
        }

        // Calculate output error
        let mut deltas = activations
            .last()
            .unwrap()
            .iter()
            .zip(targets.iter())
            .map(|(o, t)| o - t)
            .collect::<Vec<_>>();

        // Backward pass
        for (i, layer) in self.layers.iter_mut().enumerate().rev() {
            // Calculate delta for the current layer
            let delta = deltas.iter().map(|d| d * relu(*d as f32) as f64).collect::<Vec<_>>();

            // Update weights and biases
            for (j, neuron_weights) in layer.weights.iter_mut().enumerate() {
                for k in 0..neuron_weights.len() {
                    neuron_weights[k] -= self.optimizer.learning_rate as f64 * delta[j] * activations[i][k];
                }
                layer.biases[j] -= self.optimizer.learning_rate as f64 * delta[j];
            }

            // Calculate delta for the previous layer
            deltas = (0..layer.input_size).map(|k| {
                delta
                    .iter()
                    .zip(layer.weights.iter().map(|w| w[k]))
                    .map(|(d, w)| d * w)
                    .sum()
            }). collect::<Vec<_>>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlp_forward() {
        let mlp = MLP::new(&[2, 3, 1], 0.1);
        let input = vec![0.5, 0.6];
        let output = mlp.forward(&input);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_mlp_train() {
        let mut mlp = MLP::new(&[2, 3, 1], 0.1);
        let input = vec![0.5, 0.6];
        let target = vec![1.0];
        mlp.train(&input, &target);
    }
}
