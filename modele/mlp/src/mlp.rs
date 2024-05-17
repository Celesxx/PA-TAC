use crate::neural_matrix::NeuralMatrix;
use std::os::raw::{c_double, c_char};
use crate::activation::{tanh, relu, relu_derivative, tanh_derivative, softmax, softmax_derivative};
use crate::optimizer::GradientDescent;

#[derive(Debug)]
pub struct MlpModel 
{
    layer: usize,
    neural_matrix: Vec<NeuralMatrix>,
    optimizer: GradientDescent,
}


impl MlpModel 
{
    pub fn init(neurons_size: Vec<usize>, learning_rate: f64) -> Self 
    {
        let layer = neurons_size.len() - 1;
        let neural_matrix = neurons_size.windows(2).map(|w| NeuralMatrix::new(w[0], w[1])).collect();
        let optimizer = GradientDescent::new(learning_rate);

        println!("layer : {}", layer);
        println!("neural matrix : {:?}", neural_matrix);

        Self 
        {
            layer,
            neural_matrix,
            optimizer,
        }
    }

    pub fn propagate(&self, inputs: &[f64], is_classification: bool) -> Vec<f64> 
    {
        self.neural_matrix.iter().fold(inputs.to_vec(), |acc, layer| layer.forward(&acc, is_classification))
    }


    //------------------------ Passage en avant des inputs ------------------------
    pub fn forward(&self, inputs: &[f64], is_classification: bool) -> Vec<Vec<f64>> 
    {
        //Initialise l'activation
        let mut activations = vec![inputs.to_vec()];
        //Calcule les activations pour chaque couche du neuronnes
        for layer in &self.neural_matrix 
        {
            //Calcule la nouvelle sortie par rapport à la précedénte
            let output = layer.forward(activations.last().unwrap(), is_classification);
            activations.push(output);
        }
        activations
    }








    pub fn backward(&mut self, activations: &[Vec<f64>], target: &[f64], is_classification: bool) -> f64 
    {
        let last_layer_output_size = self.neural_matrix.last().unwrap().output_size;
        //Si c'est avec plus d'une classe alors utilisation de la dérivé du softmax
        //Sinon rapport diff entrée / sortie
        let mut deltas = if is_classification && last_layer_output_size > 1 {
            crate::activation::softmax_derivative(activations.last().unwrap(), &target.to_vec())
        } else {
            activations.last().unwrap().iter().zip(target.iter()).map(|(&output, &target)| output - target).collect::<Vec<_>>()
        };


        //Calcul du loss par rapport au précédent
        let mut loss = 0.0;
        for (output, &target) in activations.last().unwrap().iter().zip(target.iter()) 
        {
            loss += (output - target).powi(2);
        }
        
        //Parcours les couches neuronnes en sens inverse
        for (i, layer) in self.neural_matrix.iter_mut().enumerate().rev() 
        {

            // si une seul sortie utilisation de tanh
            // Sinon relu 
            // A revoir me semble bizarre cette histoire
            let delta = if is_classification && last_layer_output_size == 1 
            {
                deltas.iter().map(|&d| d * crate::activation::tanh_derivative(d)).collect::<Vec<_>>()
            } else {
                deltas.iter().map(|&d| d * crate::activation::relu_derivative(d)).collect::<Vec<_>>()
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




    pub fn train(&mut self, X: &[Vec<f64>], y: &[Vec<f64>], epochs: usize, is_classification: bool) -> Vec<f64> 
    {
        //Boucle pour chaque epochs 
        let mut losses = Vec::new();
        for _ in 0..epochs 
        {
            //Pour chaque valeur d'input sa boucle avec sa cible 
            let mut epoch_loss = 0.0;
            for (inputs, target) in X.iter().zip(y.iter()) 
            {
                //Passage en avant des inputs
                let activations = self.forward(inputs, is_classification);
                epoch_loss += self.backward(&activations, target, is_classification);
            }
            losses.push(epoch_loss / X.len() as f64);
        }
        losses
    }

    

    pub fn predict(&self, inputs: &[f64], is_classification: bool) -> Vec<f64> 
    {
        self.propagate(inputs, is_classification)
    }
}



#[no_mangle]
pub extern "C" fn mlpInit(neural_size: *const usize, len: usize, learning_rate: f64) -> *mut MlpModel {
    assert!(!neural_size.is_null(), "Please select a correct configuration");

    let neuron_matrix: Vec<usize> = unsafe {
        std::slice::from_raw_parts(neural_size, len).to_vec()
    };
    Box::into_raw(Box::new(MlpModel::init(neuron_matrix, learning_rate)))
}


#[no_mangle]
pub extern "C" fn mlpTrain(
    model: *mut MlpModel,
    X: *const f64,
    y: *const f64,
    n_samples: usize,
    n_features: usize,
    epochs: usize,
    is_classification: bool
) 
{
    let model = unsafe { &mut *model };
    let X = unsafe { std::slice::from_raw_parts(X, n_samples * n_features) };
    let y = unsafe { std::slice::from_raw_parts(y, n_samples) };
    let X: Vec<Vec<f64>> = X.chunks_exact(n_features).map(|chunk| chunk.to_vec()).collect();
    let y: Vec<Vec<f64>> = y.iter().map(|&val| vec![val]).collect();

    model.train(&X, &y, epochs, is_classification);
}

#[no_mangle]
pub extern "C" fn mlpPredict(
    model: *const MlpModel,
    inputs: *const f64,
    n_features: usize,
    is_classification: bool,
    predictions: *mut f64
) 
{
    assert!(!model.is_null(), "Model pointer is null");
    assert!(!inputs.is_null(), "Inputs pointer is null");
    assert!(!predictions.is_null(), "Predictions pointer is null");

    let model = unsafe { &*model };
    let inputs = unsafe { std::slice::from_raw_parts(inputs, n_features) };
    let result = model.predict(inputs, is_classification);

    for (i, &value) in result.iter().enumerate() 
    {
        unsafe {
            *predictions.add(i) = value;
        }
    }
}



#[no_mangle]
pub extern "C" fn mlpForward(
    model: *mut MlpModel,
    inputs_ptr: *const f64,
    n_inputs: usize,
    is_classification: bool
) -> *mut f64 
{
    let model = unsafe { &mut *model };
    let inputs = unsafe { std::slice::from_raw_parts(inputs_ptr, n_inputs) };

    let activations = model.forward(inputs, is_classification);
    let last_activations = activations.last().unwrap();

    let boxed_activations = last_activations.clone().into_boxed_slice();
    Box::into_raw(boxed_activations) as *mut f64
}



#[no_mangle]
pub extern "C" fn mlpBackward(
    model: *mut MlpModel,
    activations_ptr: *const f64,
    target_ptr: *const f64,
    n_layers: usize,
    is_classification: bool
) -> f64 
{
    let model = unsafe { &mut *model };
    let activations: Vec<Vec<f64>> = (0..n_layers).map(|i| {
        let ptr = unsafe { activations_ptr.add(i * n_layers) };
        let slice = unsafe { std::slice::from_raw_parts(ptr, n_layers) };
        slice.to_vec()
    }).collect();
    let target = unsafe { std::slice::from_raw_parts(target_ptr, n_layers) };

    model.backward(&activations, &target.to_vec(), is_classification)
}


#[no_mangle]
pub extern "C" fn mlpUpdateWeights(model: *mut MlpModel, learning_rate: f64) {
    let model = unsafe { &mut *model };
    model.optimizer.learning_rate = learning_rate;
}




#[no_mangle]
pub extern "C" fn mlpFree(model: *mut MlpModel) {
    if !model.is_null() {
        unsafe { Box::from_raw(model); }
    }
}







#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_mlp_model_xor() 
    {
        let neurons_size = vec![2, 3, 1];
        let learning_rate = 0.01;
        let mut model = MlpModel::init(neurons_size, learning_rate);

        let x_train = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];

        let y_train = vec![
            vec![-1.0],
            vec![1.0],
            vec![1.0],
            vec![-1.0],
        ];

        let epochs = 100000;
        model.train(&x_train, &y_train, epochs, true);

        for inputs in x_train.iter() 
        {
            let prediction = model.predict(inputs, true);
            println!("Input: {:?}, Prediction: {:?}", inputs, prediction);
        }

        let predictions: Vec<Vec<f64>> = x_train.iter().map(|inputs| model.predict(inputs, true)).collect();
        assert_eq!(predictions.len(), x_train.len());

        for (pred, target) in predictions.iter().zip(y_train.iter()) 
        {
            assert!(pred.len() == target.len());
            for (p, t) in pred.iter().zip(target.iter()) {
                assert!((p - t).abs() < 0.5, "Prediction: {:?}, Target: {:?}", pred, target);
            }
        }
    }
}

