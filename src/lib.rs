// Generics

use polars::frame::DataFrame;

trait Model
{
    type Error;
    
    fn execute(&self) -> Result<(), Self::Error>;
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

