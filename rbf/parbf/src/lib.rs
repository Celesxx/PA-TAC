extern crate nalgebra as na;
extern crate rand;

use std::os::raw::{c_double, c_char};
use std::ffi::c_void;
use rand::Rng;

pub struct RbfModel {
    n_centers: usize,
    n_features: usize,
    gamma: f64,
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
    centers: Vec<f64>,
    activation: char,
}

impl RbfModel {
    pub fn init(centers: *const f64, n_centers: usize, n_features: usize, gamma: f64, learning_rate: f64, weights: *const f64, bias: f64, activation: c_char) -> *mut RbfModel {
        let centers = unsafe { std::slice::from_raw_parts(centers, n_centers * n_features) };
        let weights = unsafe { std::slice::from_raw_parts(weights, n_centers) };
        let model = RbfModel {
            n_centers,
            n_features,
            gamma,
            learning_rate,
            weights: weights.to_vec(),
            bias,
            centers: centers.to_vec(),
            activation: activation as char,
        };
        Box::into_raw(Box::new(model))
    }

    pub fn train(&mut self, inputs: *const f64, targets: *const f64) {
        let inputs = unsafe { std::slice::from_raw_parts(inputs, self.n_features) };
        let target = unsafe { *targets };
        let output = self.predict(inputs.as_ptr());
        let error = target - output;
        for i in 0..self.n_centers {
            let center = &self.centers[i * self.n_features..(i + 1) * self.n_features];
            let phi = self.rbf(inputs, center);
            self.weights[i] += self.learning_rate * error * phi;
        }
    }

    pub fn predict(&self, inputs: *const f64) -> f64 {
        let inputs = unsafe { std::slice::from_raw_parts(inputs, self.n_features) };
        let mut output = self.bias;
        for i in 0..self.n_centers {
            let center = &self.centers[i * self.n_features..(i + 1) * self.n_features];
            let phi = self.rbf(inputs, center);
            output += self.weights[i] * phi;
        }
        output
    }

    pub fn rbf(&self, x: &[f64], c: &[f64]) -> f64 {
        let dist: f64 = x.iter().zip(c.iter()).map(|(a, b)| (a - b).powi(2)).sum();
        (-self.gamma * dist).exp()
    }
}

#[no_mangle]
pub extern "C" fn RbfModel_init(centers: *const f64, n_centers: usize, n_features: usize, gamma: f64, learning_rate: f64, weights: *const f64, bias: f64, activation: c_char) -> *mut RbfModel {
    RbfModel::init(centers, n_centers, n_features, gamma, learning_rate, weights, bias, activation)
}

#[no_mangle]
pub extern "C" fn RbfModel_train(model: *mut RbfModel, inputs: *const f64, targets: *const f64) {
    let model = unsafe { &mut *model };
    model.train(inputs, targets);
}

#[no_mangle]
pub extern "C" fn RbfModel_predict(model: *const RbfModel, inputs: *const f64) -> f64 {
    let model = unsafe { &*model };
    model.predict(inputs)
}

#[no_mangle]
pub extern "C" fn RbfModel_free(model: *mut RbfModel) {
    if !model.is_null() {
        unsafe { Box::from_raw(model) };
    }
}
