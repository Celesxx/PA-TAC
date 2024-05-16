extern crate rand;

use rand::Rng;
use std::ffi::CStr;
use std::os::raw::c_double;
use std::os::raw::c_char;
use crate::activation::sigmoid;

#[repr(C)]
pub struct LinearModel 
{
    // num_features: usize,
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
}


impl LinearModel
{
    pub fn init(learning_rate: f64, weights: Vec<f64>, bias: f64) -> Self
    {
        LinearModel 
        {
            learning_rate,
            weights,
            bias,
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
        sigmoid(sum)
    }

}


#[no_mangle]
pub extern "C" fn LM_init(learning_rate: f64, weights_ptr: *const c_double, weights_len: usize, bias: f64) -> *mut LinearModel 
{
    let weights = unsafe {
        assert!(!weights_ptr.is_null());
        std::slice::from_raw_parts(weights_ptr, weights_len).to_vec()
    };

    Box::into_raw(Box::new(LinearModel::init(learning_rate, weights, bias)))
}


#[no_mangle]
pub extern "C" fn LM_train(model: *mut LinearModel, X: *const *const c_double, y: *const c_double, n_samples: usize, n_features: usize, epochs: usize) 
{
    let model = unsafe 
    {
        assert!(!model.is_null());
        &mut *model
    };

    let X: Vec<Vec<f64>> = unsafe 
    {
        (0..n_samples)
            .map(|i| 
            {
                let row = *X.add(i);
                std::slice::from_raw_parts(row, n_features).to_vec()
            })
            .collect()
    };

    let y: Vec<f64> = unsafe { std::slice::from_raw_parts(y, n_samples).to_vec() };

    model.train(&X, &y, epochs);
}




#[no_mangle]
pub extern "C" fn LM_free(model: *mut LinearModel) 
{
    if !model.is_null() 
    {
        unsafe { Box::from_raw(model); }
    }
}