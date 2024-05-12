pub struct LinearModel 
{
    weights: Vec<f64>,
    bias: f64,
}

impl LinearModel 
{
    pub fn new(num_features: usize) -> Self 
    {
        LinearModel 
        {
            weights: vec![0.0; num_features],
            bias: 0.0,
        }
    }

    pub fn predict(&self, features: &[f64]) -> f64
    {
        self.weights.iter().zip(features).map(|(w, f)| w * f).sum::<f64>() + self.bias
    }

    pub fn train(&mut self, features: &[Vec<f64>], targets: &[f64], epochs: usize, learning_rate: f64) {
        for _ in 0..epochs 
        {
            for (input, &target) in features.iter().zip(targets) 
            {
                let predicted = self.predict(input);
                let error = predicted - target;
                for (w, &f) in self.weights.iter_mut().zip(input) 
                {
                    *w -= learning_rate * error * f;  // update poids
                }
                self.bias -= learning_rate * error;  // update biais
            }
        }
    }
}
