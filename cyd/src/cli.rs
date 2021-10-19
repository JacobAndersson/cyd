use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "hello")]
pub struct Config {
    /// Fen to start search from, if empty start pos is used
    #[clap(short, long, default_value = "")]
    pub fen: String,
    
    /// Depth to search
    #[clap(short, long, default_value = "5")]
    pub depth: u8,

    /// Number of threads to use
    #[clap(short, long, default_value = "1")]
    pub num_threads: u8,
    
    ///Moves, comma seperated
    #[clap(short, long, default_value = "")]
    pub moves: String,

    #[clap(long)]
    pub debug: bool
}

pub fn get_config() -> Config {
    let mut config = Config::parse();
    if config.fen.is_empty() {
        config.fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
    config 
}
