// tensor.rs
pub struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>,
}

impl Tensor {
    // Constructeur pour créer un tensor à partir de données et d'une forme spécifique
    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product::<usize>());
        Tensor { data, shape }
    }

    // Crée un tensor rempli de zéros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: vec![0.0; size],
            shape,
        }
    }

    // Crée un tensor rempli de uns
    pub fn ones(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: vec![1.0; size],
            shape,
        }
    }

    // Méthode pour obtenir une valeur à un index spécifique
    pub fn get(&self, index: &[usize]) -> f32 {
        let flat_index = self.calculate_flat_index(index);
        self.data[flat_index]
    }

    // Méthode pour définir une valeur à un index spécifique
    pub fn set(&mut self, index: &[usize], value: f32) {
        let flat_index = self.calculate_flat_index(index);
        self.data[flat_index] = value;
    }

    // Calcule l'index linéaire à partir d'un index multidimensionnel
    fn calculate_flat_index(&self, index: &[usize]) -> usize {
        index.iter().zip(self.shape.iter())
            .fold(0, |acc, (i, dim)| acc * dim + i)
    }
}

// Implémentation des traits Add, Mul pour les opérations entre tenseurs
use std::ops::{Add, Mul};

impl Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.shape, other.shape, "Tensors must have the same shape for addition");
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        Tensor {
            data,
            shape: self.shape,
        }
    }
}

impl Mul for Tensor {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(self.shape, other.shape, "Tensors must have the same shape for multiplication");
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).collect();
        Tensor {
            data,
            shape: self.shape,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_addition() {
        let tensor_a = Tensor::new(vec![1.0, 2.0, 3.0], vec![3]);
        let tensor_b = Tensor::new(vec![1.0, 2.0, 3.0], vec![3]);
        let result = tensor_a + tensor_b;
        assert_eq!(result.data, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_tensor_multiplication() {
        let tensor_a = Tensor::new(vec![1.0, 2.0, 3.0], vec![3]);
        let tensor_b = Tensor::new(vec![1.0, 2.0, 3.0], vec![3]);
        let result = tensor_a * tensor_b;
        assert_eq!(result.data, vec![1.0, 4.0, 9.0]);
    }
}
