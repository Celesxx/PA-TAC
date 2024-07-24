mod rbf;
use rbf::{Point, train_rbf_classification, rbf_classification, normalize_points};

fn main() {
    // Exemples de données d'entraînement et de test
    let x_train = vec![
        Point::new(vec![0.0, 0.0]),
        Point::new(vec![1.0, 1.0]),
        Point::new(vec![1.0, 0.0]),
        Point::new(vec![0.0, 1.0]),
    ];

    let y_train = vec![0.0, 1.0, 1.0, 0.0];

    let x_test = vec![
        Point::new(vec![0.5, 0.5]),
        Point::new(vec![0.2, 0.8]),
    ];

    let x_train = normalize_points(&x_train);
    let x_test = normalize_points(&x_test);

    let k = 2; // nombre de centres
    let max_iter = 100; // nombre maximum d'itérations pour l'algorithme de Lloyd
    let gamma = 1.0; // paramètre gamma pour la fonction RBF

    // Entraînement du modèle RBF pour la classification
    let (model, weights) = train_rbf_classification(&x_train, &y_train, k, max_iter, gamma);

    // Affichage des poids appris
    println!("Poids appris : {:?}", weights);

    // Prédiction sur les données de test
    let predictions = rbf_classification(&x_test, &model, gamma, &weights);

    // Affichage des prédictions
    for (i, prediction) in predictions.iter().enumerate() {
        println!("Prédiction pour x_test[{}] : {}", i, prediction);
    }
}

