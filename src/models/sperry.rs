use serde::{Serialize, Deserialize};
use crate::{Model, Output};
use polars::prelude::*;
use polars::error::PolarsError::Io;
use thiserror::Error;
use floco::Floco;

#[derive(Error, Debug, PartialEq)]
pub enum SperryError {
    #[error("an error occurred")]
    SomethingWrong()
}

pub struct SperryModel
{
    config: SperryConfig,
    data: SperryData
}

impl SperryModel {
    pub fn new(config: SperryConfig, data: SperryData) -> Self {
            Self{config, data}
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SperryConfig {
    soil: f64,
    plant: f64,
    path_to_write: std::path::PathBuf
}

impl SperryConfig {
    pub fn new(soil: f64, plant: f64, path_to_write: std::path::PathBuf) -> Self {
        Self{soil, plant, path_to_write}
    }
}

pub struct SperryData(DataFrame);

impl SperryData {
    pub fn try_new(path: std::path::PathBuf) -> Result<Self, SperryError> {
        let df = df! {
            "Foo" => [69.0f64],
            "Bar" => [4.20f64]
        };

        Ok(Self(df.unwrap()))
    }

    pub fn validate(df: DataFrame) -> bool {
        true
    }
}

impl TryFrom<std::path::PathBuf> for SperryData {
    
    type Error = SperryError;

    fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::try_new(path)
    }
}

impl Model for SperryModel {

    type Error = SperryError;

    fn execute(&self) -> Result<(), Self::Error> {
        let df = df! {
            "Foo" => [69.0f64],
            "Bar" => [4.20f64]
        }.unwrap();

        let mut out = SperryOutput::new(df);

        out.write(self.config.path_to_write.clone())
            .map_err(
                |err| 
                    Self::Error::SomethingWrong()
            )
    }
}

pub struct SperryOutput(DataFrame);

impl SperryOutput {
    pub fn new(df: DataFrame) -> Self {
        Self(df)
    }
}

impl Output for SperryOutput {
    type Error = polars::error::PolarsError;
    fn write(&mut self, path: std::path::PathBuf) -> Result<(), Self::Error> {
        let mut file = std::fs::File::create(path)?;
        CsvWriter::new(&mut file).finish(&mut self.0)?;
        Ok(())
    }
}