use clap::{ArgAction, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
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

    // FIXME do not check pretty flag every time we write...
    #[arg(long="no-pretty", short, default_value_t = true, action = ArgAction::SetFalse)]
    pub pretty: bool,

    #[arg(long, short)]
    pub count: bool,

    #[arg(long, short = 'n')]
    pub stop_after: Option<usize>,

    #[clap(long, value_name = "SHELL")]
    pub completions: Option<Shell>,
}

impl Config {
    #[cfg(test)]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(not(test))]
    pub fn new() -> Self {
        Self::parse()
    }
}

pub fn generate_completion<G: Generator>(gen: G) {
    generate(
        gen,
        &mut Config::command(),
        Config::command().get_name(),
        &mut std::io::stdout(),
    );
}
