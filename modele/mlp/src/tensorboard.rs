use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct TensorBoardLogger 
{
    file: File,
}

impl TensorBoardLogger 
{
    pub fn new(log_dir: &str) -> Self 
    {
        let log_file_path = Path::new(log_dir).join("events.out.tfevents");
        let file = File::create(log_file_path).expect("Unable to create log file");
        Self { file }
    }

    pub fn log_scalar(&mut self, tag: &str, step: u64, value: f64) {
        let log_entry = format!("{},{},{}\n", tag, step, value);
        self.file.write_all(log_entry.as_bytes()).expect("Unable to write to log file");
    }
}
