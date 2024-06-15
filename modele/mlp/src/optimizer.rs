use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct GradientDescent 
{
    pub learning_rate: f64,
}

impl GradientDescent 
{
    pub fn new(learning_rate: f64) -> Self { GradientDescent { learning_rate } }

    pub fn update(&mut self, weights: &mut [f64], gradients: &[f64]) 
    {
        for (w, g) in weights.iter_mut().zip(gradients.iter()) 
        {
            *w -= self.learning_rate * g;
        }
    }
}