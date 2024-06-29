use crate::neural_matrix::NeuralMatrix;
use crate::interop::ProgressCallback;
use std::os::raw::{c_int};
use crate::activation::{tanh_derivative, softmax_derivative};
use crate::optimizer::GradientDescent;
use crate::helpers::{generate_save_dir, generate_log_dir};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use tensorboard_rs::summary_writer::SummaryWriter;

#[derive(Debug, Serialize, Deserialize)]
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




    pub fn backward(
        &mut self, 
        activations: &[Vec<f64>], 
        target: &[f64], 
        is_classification: bool,
        weight_gradients: &mut Vec<Vec<Vec<f64>>>, 
        bias_gradients: &mut Vec<Vec<f64>>,
        is_last_chunk: bool,
        batch_len: f64
    ) -> f64 
    {
        let mut deltas = if is_classification 
        {
            if self.output_size > 1
            {
                let derivative_softmax = softmax_derivative(activations.last().unwrap());
                activations.last().unwrap().iter().zip(target.iter()).zip(derivative_softmax.iter()).map(|((&output, &target), &d_softmax)| (output - target) * d_softmax).collect::<Vec<_>>()

            }else 
            {
                activations.last().unwrap().iter().zip(target.iter()).map(|(&output, &target)| (output - target) * tanh_derivative(output)).collect::<Vec<_>>()
            }
        } else 
        {
            activations.last().unwrap().iter().zip(target.iter()).map(|(&output, &target)| output - target).collect::<Vec<_>>()
        };
    
    
        // Calcul du loss par rapport au précédent
        let mut loss = 0.0;
        for (output, &target) in activations.last().unwrap().iter().zip(target.iter()) 
        {
            loss += (output - target).powi(2);
        }
    
        let num_layers = self.neural_matrix.len();
        
        // Parcours les couches neuronnes en sens inverse
        for (i, layer) in self.neural_matrix.iter_mut().enumerate().rev() 
        {
            let delta = if i == num_layers - 1 
            {
                deltas.clone()
            } else 
            {
                if is_classification 
                {
                    deltas.iter().map(|&d| d * tanh_derivative(d)).collect::<Vec<_>>()
                } else 
                {
                    deltas.iter().map(|&d| d * tanh_derivative(d)).collect::<Vec<_>>()
                }
            };
    
            // Accumulation des gradients
            for (j, neuron_weights) in layer.matrix.iter_mut().enumerate() 
            {
                for k in 0..neuron_weights.len() 
                {
                    weight_gradients[i][j][k] += delta[j] * activations[i][k];
                    if is_last_chunk 
                    {
                        neuron_weights[k] -= self.optimizer.learning_rate * weight_gradients[i][j][k] / batch_len;
                        weight_gradients[i][j][k] = 0.0;
                    }
                }
                bias_gradients[i][j] += delta[j];
                if is_last_chunk 
                { 
                    layer.bias[j] -= self.optimizer.learning_rate * bias_gradients[i][j] / batch_len;
                    bias_gradients[i][j] = 0.0;
                }
            }
    
            // Calcule de l'erreur précédente en multipliant par le delta actuel
            deltas = (0..layer.input_size).map(|k| 
            {
                delta.iter().zip(layer.matrix.iter().map(|w| w[k])).map(|(d, w)| d * w).sum()
            }).collect::<Vec<_>>();
        }
        loss
    }
    


    
   //________________________ checkpoint save ________________________
   pub fn save(&self, filename: &str) -> Result<(), std::io::Error> 
   {
       let serialized = serde_json::to_string(&self)?;
       std::fs::write(filename, serialized)?;
       Ok(())
   }


   //________________________ checkpoint load ________________________
   pub fn load(filename: &str) -> Result<Self, std::io::Error> 
   {
       let data = std::fs::read_to_string(filename)?;
       let model: MlpModel = serde_json::from_str(&data)?;
       Ok(model)
   }

   
   

    // _________________________________ Train _________________________________
    pub fn train(
        &mut self, x: &[Vec<f64>], 
        y: &[Vec<f64>], 
        epochs: usize, 
        batch_size: usize,
        is_classification: bool,
        callback: ProgressCallback,
        callback_interval: usize,
        checkpoint_enable: bool,
        checkpoint_interval: usize,
        log_enable: bool,
        tag: &str
    )
    {
        let mut rng = thread_rng();
        let save_dir = generate_save_dir(&self.neural_matrix.iter().map(|layer| layer.output_size).collect::<Vec<usize>>(), self.optimizer.learning_rate, epochs, batch_size);
        let current_dir = env::current_dir().expect("Impossible d'obtenir le répertoire courant");
        let full_checkpoint_dir = current_dir.join("modele/save/mlp").join(save_dir.clone());
        let full_checkpoint_path = full_checkpoint_dir.as_path();
        
        let log_dir = generate_log_dir(&self.neural_matrix.iter().map(|layer| layer.output_size).collect::<Vec<usize>>(), self.optimizer.learning_rate, epochs, batch_size, tag);
        let full_log_dir = format!("modele/log/{}", log_dir);

        let mut writer = if log_enable { Some(SummaryWriter::new(&full_log_dir)) } 
        else { None };


        if checkpoint_enable
        {
            if !full_checkpoint_path.exists() 
            {
                if let Err(e) = fs::create_dir_all(&full_checkpoint_dir) 
                {
                    eprintln!("Erreur lors de la création du répertoire de sauvegarde: {}", e);
                } else {
                    println!("Répertoire de sauvegarde créé: {:?}", full_checkpoint_dir);
                }
            }
        }

        for epoch in 0..epochs 
        {
            let mut epoch_loss = 0.0;

            let mut indices: Vec<usize> = (0..x.len()).collect();
            indices.shuffle(&mut rng);


            // ------------------------------ Initialisation gradient vector ------------------------------  
            let mut weight_gradients: Vec<Vec<Vec<f64>>> = self.neural_matrix.iter()
                .map(|layer| vec![vec![0.0; layer.input_size]; layer.output_size])
                .collect();

            let mut bias_gradients: Vec<Vec<f64>> = self.neural_matrix.iter()
                .map(|layer| vec![0.0; layer.output_size])
                .collect();

            
            for batch_indices in indices.chunks(batch_size)
            {
                let batch_len = batch_indices.len() as f64;
                let mut batch_loss = 0.0;

                for (indice, &i) in batch_indices.iter().enumerate()
                {
                    let is_last_in_chunk = indice == batch_indices.len() - 1;
                    let inputs = &x[i];
                    let target = &y[i];
                    
                    let activations = self.forward(inputs, is_classification);
                    batch_loss += self.backward(&activations, target, is_classification, &mut weight_gradients, &mut bias_gradients, is_last_in_chunk, batch_len);
                }


                epoch_loss += batch_loss
            }
            
            let avg_loss = epoch_loss / x.len() as f64;

           
            if let Some(writer) = writer.as_mut() 
            {
                writer.add_scalar("loss", avg_loss as f32, epoch);
                writer.flush();
            }

            if epoch % callback_interval == 0 || epoch == epochs - 1 
            {
                callback(epoch as c_int, avg_loss);
            }

            if epoch % checkpoint_interval == 0 && checkpoint_enable || epoch == epochs && checkpoint_enable
            {
                let checkpoint_path = full_checkpoint_path.join(format!("checkpoint_epoch_{}.json", epoch));
                if let Err(e) = self.save(checkpoint_path.to_str().unwrap()) 
                {
                    eprintln!("Erreur lors de la sauvegarde du modèle: {}", e);
                } else 
                {
                    println!("Modèle sauvegardé à l'epoch {}", epoch);
                }
            }

        }
        
    }

    
    //________________________ Predict ________________________

    pub fn predict(&self, inputs: &[f64], is_classification: bool) -> Vec<f64> 
    {
        self.propagate(inputs, is_classification)
    }


}




