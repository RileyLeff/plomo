use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};
use std::fs::read_to_string;
use std::io::Write;
use polars::prelude::*;
use crate::Model;
//use floco::Floco;

pub struct SperryModel
{
    config: SperryConfig,
    data: DataFrame
}

impl SperryModel {
    pub fn new(config: SperryConfig, data: DataFrame) -> Self {
            Self{config, data}
    }

    pub fn try_new_from_paths<P: AsRef<Path>, Q: Into<PathBuf>>(config_path: P, data_path: Q) -> Result<Self, &'static str> {
        let c = SperryConfig::try_new_from_path(config_path)?;
        let d = CsvReader::from_path(data_path)
            .unwrap()
            .finish()
            .unwrap();
        Ok(Self::new(c,d))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SperryConfig {
    soil: f64,
    plant: f64
}

impl SperryConfig {
    pub fn new(soil: f64, plant: f64) -> Self {
        Self{soil, plant}
    }

    pub fn try_new_from_path<P: AsRef<Path>>(path: P) -> Result<Self, &'static str> {
        let config_str = read_to_string(path).map_err(|_| "can't read config file")?;
    
        // Deserialize TOML into Rust struct
        let config: Self = toml::from_str(&config_str).map_err(|_| "Unable to deserialize TOML")?;
        
        Ok(config)
    }

    pub fn serialize_to_path<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let toml_string = toml::to_string(self).expect("incorrectly formatted config");
        let mut file = std::fs::File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    pub fn serialize_default_to_path<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let dc = SperryConfig::default();
        dc.serialize_to_path(path)
    }
}


impl Default for SperryConfig {
    fn default() -> Self {
        SperryConfig {
            soil: 7.0f64,
            plant: 13.0f64,
        }
    }
}



impl<P: AsRef<Path>> Model<P> for SperryModel {

    type Error = &'static str;

    fn execute (&self, save_to_path: P) -> String {

        let mut df = df! {
            "Foo" => [69.0f64, 69.69f64, 69.420f64],
            "Bar" => [4.20f64, 4.269f64, 420.690f64]
        }.expect("if this fails, something very wrong");

        let mut file = std::fs::File::create(save_to_path).unwrap();
        CsvWriter::new(&mut file).finish(&mut df).unwrap();

        let message = String::from("ok!");

        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;
    use std::fs::File;

    #[test]
    fn test_serialize_to_path() {
        let config = SperryConfig::default();
        
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let temp_file_path = temp_file.path().to_string_lossy().into_owned();

        // Serialize the config to the temporary file
        assert!(config.serialize_to_path(&temp_file_path).is_ok());

        // Verify that the file content matches the serialized config
        let mut file_content = String::new();
        let mut file = File::open(temp_file_path.clone()).expect("Failed to open temporary file");
        file.read_to_string(&mut file_content).expect("Failed to read temporary file");
        assert_eq!(file_content, toml::to_string(&config).unwrap());
    }
}