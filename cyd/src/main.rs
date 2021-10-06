mod evaluate;
mod search;
mod utils;
mod cli;

extern crate clap;

fn main() {
    let config = cli::get_config();
    let (mv, score) = utils::find_move(config.moves, config.depth, config.num_threads);
    println!("{}, {}", mv, score);
}
