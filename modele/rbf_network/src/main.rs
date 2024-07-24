mod rbf;

use rbf::{Point, train_rbf_regression, rbf_regression};

fn main() {
    let x_train = vec![
        Point::new(vec![1.0, 2.0]),
        Point::new(vec![2.0, 3.0]),
        Point::new(vec![3.0, 4.0]),
    ];
    let y_train = vec![1.0, 2.0, 3.0];
    let x_test = vec![
        Point::new(vec![1.5, 2.5]),
        Point::new(vec![2.5, 3.5]),
    ];

    // Train RBF model for regression
    let (model, weights) = train_rbf_regression(&x_train, &y_train, 2, 100, 1.0);
    let predictions = rbf_regression(&x_test, &model, 1.0, &weights);
    println!("Regression predictions: {:?}", predictions);
}
