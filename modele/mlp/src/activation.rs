// activation.rs
pub fn relu(x: f32) -> f32 {
    x.max(0.0)
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        assert_eq!(relu(5.0), 5.0);
        assert_eq!(relu(-3.0), 0.0);
        assert_eq!(relu(0.0), 0.0);
    }

    #[test]
    fn test_sigmoid() {
        assert_eq!(sigmoid(0.0), 0.5);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(-10.0) < 0.01);
    }

    #[test]
    fn test_tanh() {
        assert_eq!(tanh(0.0), 0.0);
        assert!(tanh(2.0) > 0.95);
        assert!(tanh(-2.0) < -0.95);
    }
}
