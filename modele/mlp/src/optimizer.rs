#[derive(Debug)]
pub struct GradientDescent 
{
    pub learning_rate: f32,
}

impl GradientDescent 
{
    pub fn new(learning_rate: f32) -> Self { GradientDescent { learning_rate } }

    pub fn update(&mut self, weights: &mut [f64], gradients: &[f64]) 
    {
        for (w, g) in weights.iter_mut().zip(gradients.iter()) 
        {
            *w -= self.learning_rate as f64 * g;
        }
    }
}