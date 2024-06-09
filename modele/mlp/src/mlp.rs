use crate::neural_matrix::NeuralMatrix;
use crate::interop::ProgressCallback;
use std::os::raw::{c_double, c_char, c_int};
use crate::activation::{tanh, relu, relu_derivative, tanh_derivative, softmax, softmax_derivative};
use crate::optimizer::GradientDescent;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct MlpModel 
{
    layer: usize,
    neural_matrix: Vec<NeuralMatrix>,
    optimizer: GradientDescent,
    output_size: usize,
}


impl MlpModel 
{


    // _________________________________ Init _________________________________
    pub fn init(neurons_size: Vec<usize>, learning_rate: f64) -> Self 
    {
        let layer = neurons_size.len() - 1;
        let neural_matrix = neurons_size.windows(2).map(|w| NeuralMatrix::new(w[0], w[1])).collect();
        let optimizer = GradientDescent::new(learning_rate);
        let output_size = *neurons_size.last().unwrap();

        println!("layer : {}", layer);
        println!("neural matrix : {:?}", neural_matrix);

        Self 
        {
            layer,
            neural_matrix,
            optimizer,
            output_size,
        }
    }


   

    //_________________________________ Propagate _________________________________
    pub fn propagate(&self, inputs: &[f64], is_classification: bool) -> Vec<f64> 
    {
        self.neural_matrix.iter().enumerate().fold(inputs.to_vec(), |acc, (i, layer)| 
        {
            let is_output_layer = i == self.neural_matrix.len() - 1;
            layer.forward(&acc, is_classification, is_output_layer, self.output_size)
        })
    }
     



    // _________________________________ Forward _________________________________
    pub fn forward(&self, inputs: &[f64], is_classification: bool) -> Vec<Vec<f64>> 
    {
        let mut activations = vec![inputs.to_vec()];
        for (i, layer) in self.neural_matrix.iter().enumerate()
        {
            let is_output_layer = i == self.neural_matrix.len() - 1;
            let output = layer.forward(activations.last().unwrap(), is_classification, is_output_layer, self.output_size);
            activations.push(output);
        }
        activations
    }



    // _________________________________ Backward _________________________________
    pub fn backward(&mut self, activations: &[Vec<f64>], target: &[f64], is_classification: bool) -> f64 
    {
        let last_layer_output_size = self.neural_matrix.last().unwrap().output_size;
 
        let mut deltas = if is_classification
        {
            if self.output_size > 1
            {
                crate::activation::softmax_derivative(activations.last().unwrap(), &target.to_vec())
            }else
            {
                activations.last().unwrap().iter().zip(target.iter()).map(|(&output, &target)| (output - target) * crate::activation::tanh_derivative(output)).collect::<Vec<_>>()
            }
        } else 
        {
            activations.last().unwrap().iter().zip(target.iter()).map(|(&output, &target)| output - target).collect::<Vec<_>>()
        };


        //Calcul du loss par rapport au précédent
        let mut loss = 0.0;
        for (output, &target) in activations.last().unwrap().iter().zip(target.iter()) 
        {
            loss += (output - target).powi(2);
        }
        

        let num_layers = self.neural_matrix.len();
        //Parcours les couches neuronnes en sens inverse
        for (i, layer) in self.neural_matrix.iter_mut().enumerate().rev() 
        {
            let delta = if i == num_layers - 1 
            {
                deltas.clone()
            } else 
            {
                if is_classification 
                {
                    if self.output_size > 1 
                    {
                        deltas.iter().map(|&d| d * crate::activation::softmax_derivative_simple(d)).collect::<Vec<_>>()
                    } else 
                    {
                        deltas.iter().map(|&d| d * crate::activation::tanh_derivative(d)).collect::<Vec<_>>()
                    }
                } else 
                {
                    deltas.iter().map(|&d| d * crate::activation::tanh_derivative(d)).collect::<Vec<_>>()
                }
            };
            
          
            //------------------------ update des poids ------------------------
            //Parcours les couches
            for (j, neuron_weights) in layer.matrix.iter_mut().enumerate() 
            {
                //parcours les neuronnes
                for k in 0..neuron_weights.len() 
                {
                    //met à jours les poids
                    neuron_weights[k] -= self.optimizer.learning_rate * delta[j] * activations[i][k];
                }
                //Met a jour le biais
                layer.bias[j] -= self.optimizer.learning_rate * delta[j];
            }

            // Calcule de l'erreur précédente en multipliant par le delta actuel
            deltas = (0..layer.input_size).map(|k| {
                delta.iter().zip(layer.matrix.iter().map(|w| w[k])).map(|(d, w)| d * w).sum()
            }).collect::<Vec<_>>();
        }
        loss
    }


   

    // _________________________________ Train _________________________________
    pub fn train(
        &mut self, X: &[Vec<f64>], 
        y: &[Vec<f64>], 
        epochs: usize, 
        batch_size: usize,
        is_classification: bool,
        callback: ProgressCallback,
        callback_interval: usize
    )
    {
        let mut rng = thread_rng(); // Initialiser le générateur de nombres aléatoires

        for epoch in 0..epochs 
        {
            let mut epoch_loss = 0.0;

            // Créer un vecteur d'indices et le mélanger
            let mut indices: Vec<usize> = (0..X.len()).collect();
            indices.shuffle(&mut rng);

            // Utiliser les indices mélangés pour accéder aux éléments de X et y
            // for &i in indices.iter() 
            for batch_indices in indices.chunks(batch_size)
            // for (inputs, target) in X.iter().zip(y.iter())
            {
                let mut batch_loss = 0.0;
                // let inputs = &X[i];
                // let target = &y[i];

                for &i in batch_indices 
                {
                    let inputs = &X[i];
                    let target = &y[i];

                    let activations = self.forward(inputs, is_classification);
                    batch_loss += self.backward(&activations, target, is_classification);
                }

                // let activations = self.forward(inputs, is_classification);
                // epoch_loss += self.backward(&activations, target, is_classification);
                epoch_loss += batch_loss / batch_indices.len() as f64;
            }
            // let avg_loss = epoch_loss / X.len() as f64;
            // if epoch % callback_interval == 0 || epoch == epochs - 1 
            // {
            //     callback(epoch as c_int, avg_loss);
            // }
            
            let num_batches = if X.len() / batch_size == 0 { 1 } else { X.len() / batch_size };
            let avg_loss = epoch_loss / num_batches as f64;
            if epoch % callback_interval == 0 || epoch == epochs - 1 
            {
                callback(epoch as c_int, avg_loss);
            }
        }
    }

    

    pub fn predict(&self, inputs: &[f64], is_classification: bool) -> Vec<f64> 
    {
        self.propagate(inputs, is_classification)
    }
}




