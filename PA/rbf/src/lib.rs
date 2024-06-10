extern crate rand;

use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_char};

#[repr(C)]
pub struct RbfModel {
    centers: Vec<f64>,
    n_centers: usize,
    n_features: usize,
    gamma: f64,
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
    activation: char,
}

impl RbfModel {
    pub fn init(
        centers: &[f64],
        n_centers: usize,
        n_features: usize,
        gamma: f64,
        learning_rate: f64,
        weights: &[f64],
        bias: f64,
        activation: char,
    ) -> Self {
        Self {
            centers: centers.to_vec(),
            n_centers,
            n_features,
            gamma,
            learning_rate,
            weights: weights.to_vec(),
            bias,
            activation,
        }
    }

    fn rbf(&self, x: &[f64], c: &[f64]) -> f64 {
        let dist_sq: f64 = x.iter().zip(c).map(|(xi, ci)| (xi - ci).powi(2)).sum();
        let result = (-self.gamma * dist_sq).exp();
        println!("rbf: x = {:?}, c = {:?}, dist_sq = {}, result = {}", x, c, dist_sq, result);
        result
    }

    pub fn predict(&self, inputs: &[f64]) -> f64 {
        let mut output = self.bias;
        for i in 0..self.n_centers {
            let center = &self.centers[i * self.n_features..(i + 1) * self.n_features];
            let phi = self.rbf(inputs, center);
            output += self.weights[i] * phi;
        }
        println!("predict: inputs = {:?}, output = {}", inputs, output);
        output
    }

    pub fn train(&mut self, inputs: &[f64], target: &f64) {
        let prediction = self.predict(inputs);
        let error = target - prediction;
        println!("train: inputs = {:?}, target = {}, prediction = {}, error = {}", inputs, target, prediction, error);

        for i in 0..self.n_centers {
            let center = &self.centers[i * self.n_features..(i + 1) * self.n_features];
            let phi = self.rbf(inputs, center);
            self.weights[i] += self.learning_rate * error * phi;
            println!("train: center = {:?}, phi = {}, new weight = {}", center, phi, self.weights[i]);
        }
        self.bias += self.learning_rate * error;
        println!("train: new bias = {}", self.bias);
    }
}

#[no_mangle]
pub extern "C" fn RbfModel_init(
    centers_ptr: *const c_double,
    n_centers: usize,
    n_features: usize,
    gamma: c_double,
    learning_rate: c_double,
    weights_ptr: *const c_double,
    bias: c_double,
    activation: c_char,
) -> *mut RbfModel {
    let centers = unsafe { std::slice::from_raw_parts(centers_ptr, n_centers * n_features) };
    let weights = unsafe { std::slice::from_raw_parts(weights_ptr, n_centers) };
    let model = RbfModel::init(centers, n_centers, n_features, gamma, learning_rate, weights, bias, activation as char);
    Box::into_raw(Box::new(model))
}

#[no_mangle]
pub extern "C" fn RbfModel_train(
    model_ptr: *mut RbfModel,
    inputs_ptr: *const c_double,
    target_ptr: *const c_double,
) {
    let model = unsafe { &mut *model_ptr };
    let inputs = unsafe { std::slice::from_raw_parts(inputs_ptr, model.n_features) };
    let target = unsafe { &*target_ptr };
    model.train(inputs, target);
}

#[no_mangle]
pub extern "C" fn RbfModel_predict(
    model_ptr: *const RbfModel,
    inputs_ptr: *const c_double,
) -> c_double {
    let model = unsafe { &*model_ptr };
    let inputs = unsafe { std::slice::from_raw_parts(inputs_ptr, model.n_features) };
    model.predict(inputs)
}

#[no_mangle]
pub extern "C" fn RbfModel_free(model_ptr: *mut RbfModel) {
    if !model_ptr.is_null() {
        unsafe { Box::from_raw(model_ptr); }
    }
}
