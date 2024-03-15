use polars::frame::DataFrame;
use thiserror::Error;
use std::path::Path;

#[derive(Error, Debug)]
pub enum ModelIdError<'a> {
    #[error("unknown model ID `{0}`.")]
    UnknownModel(&'a str)
}

pub trait Model<P: AsRef<Path>>
{
    type Error;
    fn execute(&self, path: P) -> String;
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

