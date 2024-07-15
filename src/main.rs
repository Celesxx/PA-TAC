mod rbf;
use rbf::{Point, train_rbf_regression, rbf_regression};

fn main() {
    let x_train = vec![
        Point::new(vec![1.0, 1.0]),
        Point::new(vec![2.0, 3.0]),
        Point::new(vec![3.0, 3.0]),
    ];
    let y_train = vec![2.0, 3.0, 3.0];
    let x_test = vec![
        Point::new(vec![1.0, 1.0]),
        Point::new(vec![2.0, 3.0]),
        Point::new(vec![3.0, 3.0]),
    ];
    let y_true = vec![2.0, 3.0, 3.0];

    let gamma = 0.1;
    let num_centers = 3;
    let max_iter = 100;

    let (model, weights) = train_rbf_regression(&x_train, &y_train, num_centers, max_iter, gamma);
    let y_pred = rbf_regression(&x_test, &model, gamma, &weights);

    println!("True values: {:?}", y_true);
    println!("Predictions: {:?}", y_pred);
}
