use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};
use std::fs::read_to_string;
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