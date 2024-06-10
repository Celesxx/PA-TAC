use rbf_model::{RbfModel_init, RbfModel_train, RbfModel_predict, RbfModel_free};
use std::os::raw::c_double;

fn main() {
    // Exemple de centres pour les fonctions RBF
    let centers: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let centers_flatten: Vec<f64> = centers.iter().flatten().cloned().collect();
    let n_centers = centers.len();
    let n_features = centers[0].len();

    // Hyperparamètres
    let gamma = 1.0;
    let learning_rate = 0.01;
    let weights: Vec<f64> = vec![0.0; n_centers];
    let bias = 0.0;
    let activation = 'r'; // ReLU

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
            activation as u8,
        )
    };

    // Utilisez rbf_model pour l'entraînement, la prédiction, etc.
}
