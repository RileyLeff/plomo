use polars::frame::DataFrame;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelIdError<'a> {
    #[error("unknown model ID `{0}`.")]
    UnknownModel(&'a str)
}

trait Model
{
    type Error;
    fn execute(&self) -> Result<impl Output, Self::Error>;
}

trait Output {
    type Error;
    fn write(&mut self, path: std::path::PathBuf) -> Result<(),Self::Error>;
}

pub mod models;



// trait Config<M>
// where
//     M: Model
// {
//     fn validate() -> bool;
// }

// trait Data<M>
// where
//     M: Model {
//     fn validate() -> bool;
// }

// Real Shit

