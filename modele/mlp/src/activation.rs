pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

pub fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

pub fn tanh_derivative(x: f64) -> f64 {
    1.0 - x.tanh().powi(2)
}

pub fn softmax(x: &Vec<f64>) -> Vec<f64> {
    let max = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_x: Vec<f64> = x.iter().map(|&xi| (xi - max).exp()).collect();
    let sum_exp_x: f64 = exp_x.iter().sum();
    exp_x.iter().map(|&xi| xi / sum_exp_x).collect()
}

pub fn softmax_derivative(output: &Vec<f64>, target: &Vec<f64>) -> Vec<f64> {
    output.iter().zip(target.iter()).map(|(o, t)| o - t).collect()
}


pub fn softmax_derivative_simple(output: f64) -> f64 {
    output * (1.0 - output)
}

