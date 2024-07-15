use ndarray::{Array2, ArrayView1, ArrayView2};
use ndarray_linalg::Inverse;
use std::f64::consts::E;

#[derive(Clone)]
pub struct Point {
    pub data: Vec<f64>,
}

pub struct RBFModel {
    pub centers: Vec<Point>,
}

impl Point {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

pub fn lloyd_algorithm(points: &[Point], k: usize, max_iter: usize) -> Vec<Point> {
    let mut centers = initialize_centers(points, k);
    let mut assignments = vec![0; points.len()];

    for _ in 0..max_iter {
        let mut new_centers = vec![vec![0.0; points[0].data.len()]; k];
        let mut counts = vec![0; k];

        for (i, point) in points.iter().enumerate() {
            let (min_index, _) = centers.iter()
                .enumerate()
                .map(|(j, center)| (j, point.euclidean_distance(center)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            assignments[i] = min_index;
            counts[min_index] += 1;
            for (j, &val) in point.data.iter().enumerate() {
                new_centers[min_index][j] += val;
            }
        }

        let mut converged = true;
        for (i, center) in centers.iter_mut().enumerate() {
            for j in 0..center.data.len() {
                let new_center = new_centers[i][j] / counts[i] as f64;
                if (new_center - center.data[j]).abs() > 1e-4 {
                    converged = false;
                }
                center.data[j] = new_center;
            }
        }

        if converged {
            break;
        }
    }

    centers
}

fn initialize_centers(points: &[Point], k: usize) -> Vec<Point> {
    let mut centers = Vec::with_capacity(k);
    for _ in 0..k {
        let data = points[rand::random::<usize>() % points.len()].data.clone();
        centers.push(Point::new(data));
    }
    centers
}

pub fn train_rbf_model(points: &[Point], k: usize, max_iter: usize) -> RBFModel {
    let centers = lloyd_algorithm(points, k, max_iter);
    RBFModel { centers }
}

pub fn rbf(x: &[f64], c: &[f64], gamma: f64) -> f64 {
    let dist: f64 = x.iter().zip(c).map(|(xi, ci)| (xi - ci).powi(2)).sum();
    E.powf(-gamma * dist)
}

pub fn compute_rbf_matrix(x_train: &[Point], model: &RBFModel, gamma: f64) -> Array2<f64> {
    let n_train = x_train.len();
    let k = model.centers.len();
    let mut rbf_matrix = Array2::zeros((n_train, k));

    for i in 0..n_train {
        for j in 0..k {
            rbf_matrix[[i, j]] = rbf(&x_train[i].data, &model.centers[j].data, gamma);
        }
    }

    rbf_matrix
}

pub fn compute_pseudo_inverse(matrix: ArrayView2<f64>) -> Array2<f64> {
    let temp = matrix.t().dot(&matrix) + Array2::<f64>::eye(matrix.ncols()) * 1e-8f64;
    temp.inv().unwrap().dot(&matrix.t())
}

pub fn train_rbf_regression(
    x_train: &[Point],
    y_train: &[f64],
    k: usize,
    max_iter: usize,
    gamma: f64,
) -> (RBFModel, Vec<f64>) {
    let model = train_rbf_model(x_train, k, max_iter);
    let rbf_matrix = compute_rbf_matrix(x_train, &model, gamma);
    let pseudo_inverse = compute_pseudo_inverse(rbf_matrix.view());
    let weights = pseudo_inverse.dot(&ArrayView1::from(y_train)).to_vec();

    (model, weights)
}

pub fn rbf_regression(
    x_test: &[Point],
    model: &RBFModel,
    gamma: f64,
    weights: &[f64],
) -> Vec<f64> {
    let rbf_matrix = compute_rbf_matrix(x_test, model, gamma);
    rbf_matrix.dot(&ArrayView1::from(weights)).to_vec()
}
