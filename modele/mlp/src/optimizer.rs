// optimizer.rs

pub struct GradientDescent {
    pub learning_rate: f32,
}

pub struct MomentumGD {
    pub learning_rate: f32,
    pub momentum: f32,
    pub velocity: Vec<f32>,
}

impl GradientDescent {
    pub fn new(learning_rate: f32) -> Self {
        GradientDescent { learning_rate }
    }

    pub fn update(&mut self, weights: &mut [f32], gradients: &[f32]) {
        for (w, g) in weights.iter_mut().zip(gradients.iter()) {
            *w -= self.learning_rate * g;
        }
    }
}

impl MomentumGD {
    pub fn new(learning_rate: f32, momentum: f32, initial_weights_size: usize) -> Self {
        MomentumGD {
            learning_rate,
            momentum,
            velocity: vec![0.0; initial_weights_size],
        }
    }

    pub fn update(&mut self, weights: &mut [f32], gradients: &[f32]) {
        for (i, (w, g)) in weights.iter_mut().zip(gradients.iter()).enumerate() {
            self.velocity[i] = self.momentum * self.velocity[i] + self.learning_rate * g;
            *w -= self.velocity[i];
        }
    }
}

// optimizer.rs

// Existing optimizer code here...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_descent() {
        let mut weights = vec![0.5, -0.5];
        let gradients = vec![0.1, -0.1];
        let mut optimizer = GradientDescent::new(0.1);

        optimizer.update(&mut weights, &gradients);

        assert_eq!(weights, vec![0.5 - 0.1 * 0.1, -0.5 + 0.1 * 0.1]);
    }

    // optimizer.rs

// Existing optimizer code here...

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_gradient_descent() {
            let mut weights = vec![0.5, -0.5];
            let gradients = vec![0.1, -0.1];
            let mut optimizer = GradientDescent::new(0.1);

            optimizer.update(&mut weights, &gradients);

            assert_eq!(weights, vec![0.5 - 0.1 * 0.1, -0.5 + 0.1 * 0.1]);
        }


        #[test]
        fn test_momentum_gd() {
            let mut weights = vec![0.5, -0.5];
            let gradients = vec![0.1, -0.1];
            let mut optimizer = MomentumGD::new(0.1, 0.9, 2);

            optimizer.update(&mut weights, &gradients);
            optimizer.update(&mut weights, &gradients);  // Update twice to see the effect of momentum

            // Define a small tolerance for floating-point comparison
            let tolerance = 0.01;
            let expected_weights = vec![0.481, -0.481];

            assert!(weights.iter().zip(expected_weights.iter()).all(|(a, b)| (a - b).abs() < tolerance),
                    "Weights did not update within the expected tolerance: {:?} != {:?}", weights, expected_weights);
        }
    }
}
