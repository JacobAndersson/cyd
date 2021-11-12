//use clap::Clap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hello")]
pub struct Config {
    /// Fen to start search from, if empty start pos is used
    #[structopt(short, long, default_value = "")]
    pub fen: String,

    /// Depth to search
    #[structopt(short, long, default_value = "5")]
    pub depth: u8,

    /// Number of threads to use
    #[structopt(short, long, default_value = "1")]
    pub num_threads: u8,

    ///Moves, comma seperated
    #[structopt(short, long, default_value = "")]
    pub moves: String,

    #[structopt(long)]
    pub debug: bool,

    #[structopt(long)]
    pub alive: bool,
}

pub fn get_config() -> Config {
    let mut config = Config::from_args();
    if config.fen.is_empty() {
        config.fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
    config
}
