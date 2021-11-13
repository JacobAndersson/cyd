#[allow(clippy::module_inception)]
mod search;
mod search_test;
pub mod transposition_table;

pub use search::*;
