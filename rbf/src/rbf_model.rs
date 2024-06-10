extern crate rand;

use std::os::raw::{c_char, c_double};

use crate::linear_model::LinearModel;

#[repr(C)]
pub struct RbfModel {
    pub centers: Vec<Vec<f64>>,
    pub gamma: f64,
    pub linear_model: LinearModel,
}

impl RbfModel {
    pub fn init(centers: Vec<Vec<f64>>, gamma: f64, learning_rate: f64, weights: Vec<f64>, bias: f64, activation: char) -> Self {
        let linear_model = LinearModel::init(learning_rate, weights, bias, activation);
        RbfModel { centers, gamma, linear_model }
    }

    fn rbf_transform(&self, inputs: &Vec<f64>) -> Vec<f64> {
        self.centers.iter().map(|center| {
            let sum = center.iter().zip(inputs.iter()).map(|(c, x)| (c - x).powi(2)).sum::<f64>();
            (-self.gamma * sum).exp()
        }).collect()
    }

    pub fn train(&mut self, x: &Vec<Vec<f64>>, y: &Vec<f64>, epochs: usize) {
        let transformed_x: Vec<Vec<f64>> = x.iter().map(|inputs| self.rbf_transform(inputs)).collect();
        self.linear_model.train(&transformed_x, y, epochs);
    }

    pub fn predict(&self, inputs: &Vec<f64>) -> f64 {
        let transformed_inputs = self.rbf_transform(inputs);
        self.linear_model.predict(&transformed_inputs)
    }
}

#[no_mangle]
pub extern "C" fn RbfModel_init(centers_ptr: *const c_double, n_centers: usize, n_features: usize, gamma: f64, learning_rate: f64, weights_ptr: *const c_double, weights_len: usize, bias: f64, activation: c_char) -> *mut RbfModel {
    let centers = unsafe { std::slice::from_raw_parts(centers_ptr, n_centers * n_features) }
        .chunks(n_features)
        .map(|chunk| chunk.to_vec())
        .collect();
    let weights = unsafe { std::slice::from_raw_parts(weights_ptr, weights_len).to_vec() };
    let activation_char = activation as u8 as char;
    Box::into_raw(Box::new(RbfModel::init(centers, gamma, learning_rate, weights, bias, activation_char)))
}

#[no_mangle]
pub extern "C" fn RbfModel_train(model: *mut RbfModel, x: *const c_double, y: *const c_double, n_samples: usize, n_features: usize, epochs: usize) {
    let model = unsafe { &mut *model };
    let x = unsafe { std::slice::from_raw_parts(x, n_samples * n_features) };
    let y = unsafe { std::slice::from_raw_parts(y, n_samples) };
    let x: Vec<Vec<f64>> = x.chunks_exact(n_features).map(|chunk| chunk.to_vec()).collect();
    let y: Vec<f64> = y.to_vec();
    model.train(&x, &y, epochs);
}

#[no_mangle]
pub extern "C" fn RbfModel_predict(model: *const RbfModel, inputs: *const c_double, n_features: usize) -> f64 {
    let model = unsafe { &*model };
    let inputs = unsafe { std::slice::from_raw_parts(inputs, n_features) };
    model.predict(&inputs.to_vec())
}

#[no_mangle]
pub extern "C" fn RbfModel_free(model: *mut RbfModel) {
    if !model.is_null() {
        unsafe { Box::from_raw(model); }
    }
}
