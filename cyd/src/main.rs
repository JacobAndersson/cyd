mod cli;
mod evaluate;
mod search;
mod utils;

extern crate clap;

fn main() {
    let config = cli::get_config();
    if config.debug {
        utils::from_start(config.depth, config.num_threads);
    } else {
        let (mv, score) = utils::find_move(config.moves, config.depth, config.num_threads);
        println!("{}, {}", mv, score);
    }
}
