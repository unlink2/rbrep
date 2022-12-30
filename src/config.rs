use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    pub expr: String,

    pub paths: Vec<String>,

    #[arg(long, default_value_t = false)]
    pub dbg_expr_tree: bool,

    #[arg(long, short, default_value_t = 1)]
    pub space: u32,

    #[arg(long, short)]
    pub count: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
}
