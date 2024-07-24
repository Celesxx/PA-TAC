
use chrono::Local;

//________________________ generate folder checkpoint ________________________
pub fn generate_save_dir(layers: &[usize], learning_rate: f64, epochs: usize, batch_size: usize) -> String 
{
    let layers_str = layers.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(".");
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    format!("modele_layer{}_learning{}_epoch{}_batch{}_{}", layers_str, learning_rate, epochs, batch_size, timestamp)
}

pub fn generate_log_dir(layers: &[usize], learning_rate: f64, epochs: usize, batch_size: usize, tag: &str) -> String 
{
    let layers_str = layers.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(".");
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    format!("mlp_{}_l{}_lr{}_e{}_b{}_{}", tag, layers_str, learning_rate, epochs, batch_size, timestamp)
}
