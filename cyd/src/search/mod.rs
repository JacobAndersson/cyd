#[allow(clippy::module_inception)]
mod search;
mod search_test;
mod timer;
pub mod transposition_table;

pub use search::*;
pub use timer::Timer;
