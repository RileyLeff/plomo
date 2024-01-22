use crate::Model;
use polars::prelude::*;
use thiserror::Error;
use floco::Floco;

#[derive(Error, Debug, PartialEq)]
pub enum SperryError {
    #[error("an error occurred")]
    SomethingWrong()
}

struct SperryModel
{
    config: SperryConfig,
    data: SperryData
}

struct SperryConfig {
    soil: f64,
    plant: f64
}

struct SperryData(DataFrame);

impl SperryData {
    fn try_new(path: std::path::PathBuf) -> Result<Self, SperryError> {
        let df = df! {
            "Foo" => [69.0f64],
            "Bar" => [4.20f64]
        };

        Ok(Self(df.unwrap()))
    }

    fn validate(df: DataFrame) -> bool {
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
    fn execute() -> bool {
        true
    }
}