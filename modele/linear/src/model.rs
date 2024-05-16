extern crate rand;

use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_char};
use crate::activation::{sigmoid, tanh, relu};

#[repr(C)]
pub struct LinearModel 
{
    // num_features: usize,
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
    activation: char,
}


impl LinearModel
{
    pub fn init(learning_rate: f64, weights: Vec<f64>, bias: f64, activation: char) -> Self
    {
        LinearModel 
        {   
            learning_rate,
            weights,
            bias,
            activation,
        }
    }

    //Fonction entrainement
    pub fn train(&mut self, X: &Vec<Vec<f64>>, y: &Vec<f64>, epochs: usize) 
    {
        for _ in 0..epochs 
        {
            for (inputs, &target) in X.iter().zip(y.iter()) 
            {
                let prediction = self.predict(inputs);
                let error = prediction - target;
                self.update_weights(inputs, error);
            }
        }
    }

    //Gradient
    fn update_weights(&mut self, inputs: &Vec<f64>, error: f64) 
    {
        for (weight, input) in self.weights.iter_mut().zip(inputs.iter()) 
        {
            *weight -= self.learning_rate * error * input;
        }
        self.bias -= self.learning_rate * error;
    }


    //Fonction prediction
    pub fn predict(&self, inputs: &Vec<f64>) -> f64 
    {
        let mut sum = self.bias;
        for (weight, input) in self.weights.iter().zip(inputs.iter()) 
        {
            sum += weight * input;
        }

        match self.activation 
        {
            't' => tanh(sum),
            'r' => relu(sum),
            _ => sigmoid(sum),
        }
    }

}


#[no_mangle]
pub extern "C" fn LM_init(learning_rate: f64, weights_ptr: *const c_double, weights_len: usize, bias: f64, activation: c_char) -> *mut LinearModel 
{
    let weights = unsafe 
    {
        assert!(!weights_ptr.is_null());
        std::slice::from_raw_parts(weights_ptr, weights_len).to_vec()
    };

    let activation_char = activation as u8 as char;
    let valid_activations = ['t', 's', 'r'];
    assert!(
        valid_activations.contains(&activation_char),
        "Invalid activation function, select one of them (t for tanh, s for sigmoid, r for relu)"
    );
    Box::into_raw(Box::new(LinearModel::init( learning_rate, weights, bias, activation_char, )))
}



#[no_mangle]
pub extern "C" fn LM_train(
    model: *mut LinearModel,
    X: *const c_double,
    y: *const c_double,
    n_samples: usize,
    n_features: usize,
    epochs: usize,
)
{
    //vérification des paramètres
    assert!(!model.is_null(), "Model pointer is null");
    assert!(!X.is_null(), "X pointer is null");
    assert!(!y.is_null(), "y pointer is null");

    // Conversion du pointeur modèle en référence mutable
    let model = unsafe { &mut *model };

    // Conversion du <vec> en <vec<vec>> à partir du <vec> flatten et la taille
    let x_vector = unsafe { std::slice::from_raw_parts(X, n_samples * n_features) };
    let mut x_converted = Vec::with_capacity(n_samples);
    for i in 0..n_samples 
    {
        let start = i * n_features;
        let end = start + n_features;
        let row = &x_vector[start..end];
        x_converted.push(row.to_vec());
    }

    let y_vector: Vec<f64> = unsafe { std::slice::from_raw_parts(y, n_samples).to_vec() };

    // Entraînez le modèle
    for epoch in 0..epochs 
    {
        model.train(&x_converted, &y_vector, 1);
        if epoch % 100 == 0 { println!("Epoch {} completed", epoch); }
    }
}


#[no_mangle]
pub extern "C" fn LM_predict(
    model: *const LinearModel,
    x: *const c_double,
    n_samples: usize,
    n_features: usize,
    predictions: *mut c_double,
) 
{
    assert!(!model.is_null(), "Model pointer is null");
    assert!(!x.is_null(), "x pointer is null");
    assert!(!predictions.is_null(), "Predictions pointer is null");
    let model = unsafe { &*model };

    let x_vector = unsafe { std::slice::from_raw_parts(x, n_samples * n_features) };

    for i in 0..n_samples 
    {
        let start = i * n_features;
        let end = start + n_features;
        let row = &x_vector[start..end];
        let prediction = model.predict(&row.to_vec());
        unsafe {
            *predictions.add(i) = prediction;
        }
    }
}


#[no_mangle]
pub extern "C" fn LM_free(model: *mut LinearModel) 
{
    if !model.is_null() 
    {
        unsafe { Box::from_raw(model); }
    }
}