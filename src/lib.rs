mod args;
mod date;
mod error;
mod run;

pub use date::{Advent, Day, Year};
pub use error::AdventError as Error;
pub use run::entrypoint;
