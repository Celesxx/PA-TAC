extern crate rbf_model;

use rbf_model::{RbfModel_init, RbfModel_train, RbfModel_predict, RbfModel_free};
use std::os::raw::c_double;

fn main() {
    // Exemple de centres pour les fonctions RBF
    let centers: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0],
        vec![1.0, 1.0],
    ];

    let centers_flatten: Vec<f64> = centers.iter().flatten().cloned().collect();
    let n_centers: usize = centers.len();
    let n_features: usize = centers[0].len();

    // Hyperparamètres
    let gamma: f64 = 1.0;
    let learning_rate: f64 = 0.01;
    let weights: Vec<f64> = vec![0.0; n_centers];
    let bias: f64 = 0.0;
    let activation: u8 = 'r' as u8; // ReLU

    // Initialisation du modèle RBF
    let rbf_model = unsafe {
        RbfModel_init(
            centers_flatten.as_ptr() as *const c_double,
            n_centers,
            n_features,
            gamma,
            learning_rate,
            weights.as_ptr() as *const c_double,
            bias,
            activation as c_char,
        )
    };

    // Exemple d'entraînement
    let inputs: Vec<f64> = vec![0.5, 0.5];
    let target: f64 = 1.0;
    unsafe {
        RbfModel_train(rbf_model, inputs.as_ptr() as *const c_double, &target as *const f64);
    }

    // Exemple de prédiction
    let prediction = unsafe {
        RbfModel_predict(rbf_model, inputs.as_ptr() as *const c_double)
    };
    println!("Prédiction: {}", prediction);

    // Libérer la mémoire du modèle RBF
    unsafe {
        RbfModel_free(rbf_model);
    }
}
