pub mod loss_functions 
{

    /// erreur quadratique moyenne (Mean Squared Error - MSE)
    /// `y_true` : Vecteur des valeurs réelles
    /// `y_pred` : Vecteur des valeurs prédites
    pub fn mean_squared_error(y_true: &[f64], y_pred: &[f64]) -> f64 
    {
        let n = y_true.len();
        y_true.iter().zip(y_pred.iter())
            .map(|(yt, yp)| (yt - yp).powi(2))
            .sum::<f64>() / n as f64
    }

    /// erreur absolue moyenne (Mean Absolute Error - MAE)
    /// `y_true` : Vecteur des valeurs réelles
    /// `y_pred` : Vecteur des valeurs prédites
    pub fn mean_absolute_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
        let n = y_true.len();
        y_true.iter().zip(y_pred.iter())
            .map(|(yt, yp)| (yt - yp).abs())
            .sum::<f64>() / n as f64
    }

}
