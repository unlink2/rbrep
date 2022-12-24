use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    pub expr: String,

    pub paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            expr: "".into(),
            paths: vec![],
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn exec(&mut self) {}

    pub fn finish(&mut self) {}
}
