use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    expr: String,

    paths: Vec<String>,
}

impl Config {
    pub fn exec(&mut self) {}

    pub fn finish(&mut self) {}
}
