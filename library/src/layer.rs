// Définition de la structure de la couche dense
pub struct DenseLayer {
    pub(crate) weights: Vec<Vec<f64>>,  // Poids de la couche
    pub(crate) biases: Vec<f64>,         // Biais de la couche
    pub(crate) input_size: usize,        // Taille de l'entrée
    output_size: usize,       // Taille de la sortie
}

// Implémentation des méthodes pour la couche dense
impl DenseLayer {    // Fonction de création d'une nouvelle couche dense
    pub fn new(input_size: usize, output_size: usize) -> Self {
        // Initialisation aléatoire des poids et des biais
        let mut weights = Vec::with_capacity(output_size);
        for _ in 0..output_size {
            let mut neuron_weights = Vec::with_capacity(input_size);
            for _ in 0..input_size {
                neuron_weights.push(rand::random()); // Initialisation aléatoire
            }
            weights.push(neuron_weights);
        }

        let mut biases = Vec::with_capacity(output_size);
        for _ in 0..output_size {
            biases.push(rand::random()); // Initialisation aléatoire
        }

        DenseLayer {
            weights,
            biases,
            input_size,
            output_size,
        }
    }

    // Fonction pour effectuer la propagation avant à travers la couche
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        assert_eq!(input.len(), self.input_size);

        let mut output = Vec::with_capacity(self.output_size);
        for neuron_weights in &self.weights {
            let mut neuron_output = 0.0;
            for (input_val, weight) in input.iter().zip(neuron_weights) {
                neuron_output += input_val * weight;
            }
            output.push(neuron_output);
        }

        output.iter_mut().zip(&self.biases).for_each(|(o, b)| *o += b);
        output
    }
}


        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_dense_layer_creation() {
                let input_size = 3;
                let output_size = 2;
                let layer = DenseLayer::new(input_size, output_size);

                // Vérifiez que les poids et les biais ont la bonne taille
                assert_eq!(layer.weights.len(), output_size);
                assert_eq!(layer.biases.len(), output_size);

                for neuron_weights in &layer.weights {
                    assert_eq!(neuron_weights.len(), input_size);
                }
            }

            #[test]
            fn test_dense_layer_forward() {
                // Créez une couche dense avec des poids et des biais prédéfinis
                let weights = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
                let biases = vec![0.1, 0.2];
                let input_size = 3;
                let output_size = 2;
                let layer = DenseLayer { weights, biases, input_size, output_size };

                // Appliquez la propagation avant avec une entrée donnée
                let input = vec![0.5, 0.6, 0.7];
                let output = layer.forward(&input);

                // Vérifiez que la sortie a la bonne taille
                assert_eq!(output.len(), output_size);

                // Vérifiez le calcul de la sortie
                let expected_output_0 = 0.5 * 1.0 + 0.6 * 2.0 + 0.7 * 3.0 + 0.1;
                let expected_output_1 = 0.5 * 4.0 + 0.6 * 5.0 + 0.7 * 6.0 + 0.2;
                assert_eq!(output[0], expected_output_0);
                assert_eq!(output[1], expected_output_1);
            }
        }

