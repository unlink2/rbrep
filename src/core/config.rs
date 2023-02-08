use std::path::PathBuf;

#[cfg(feature = "cli")]
use clap::{ArgAction, CommandFactory, Parser};
#[cfg(feature = "cli")]
use clap_complete::{generate, Generator, Shell};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "cli", derive(Parser))]
#[cfg_attr(feature = "cli", command(author, version, about, long_about = None))]
pub struct Config {
    pub expr: String,

    pub paths: Vec<PathBuf>,

    #[cfg_attr(feature = "cli", arg(long, default_value_t = false))]
    pub dbg_expr_tree: bool,

    #[cfg_attr(feature = "cli", arg(long, short, default_value_t = 1))]
    pub space: u32,

    // FIXME do not check pretty flag every time we write...
    #[cfg_attr(feature = "cli", arg(long="no-pretty", short, default_value_t = true, action = ArgAction::SetFalse))]
    pub pretty: bool,

    #[cfg_attr(feature = "cli", arg(long, short))]
    pub count: bool,

    #[cfg_attr(feature = "cli", arg(long, short = 'n'))]
    pub stop_after: Option<usize>,

    #[cfg_attr(feature = "cli", clap(long, value_name = "SHELL"))]
    #[cfg(feature = "cli")]
    pub completions: Option<Shell>,
}

impl Config {
    #[cfg(all(feature = "cli", not(test)))]
    pub fn new() -> Self {
        Self::parse()
    }
    #[cfg(any(test, not(feature = "cli")))]
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(feature = "cli")]
pub fn generate_completion<G: Generator>(gen: G) {
    generate(
        gen,
        &mut Config::command(),
        Config::command().get_name(),
        &mut std::io::stdout(),
    );
}
