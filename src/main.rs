#[cfg(not(any(feature = "cli")))]
fn main() {}

#[cfg(feature = "cli")]
fn main() {
    use rbrep::{core::config::generate_completion, prelude::exec, prelude::CFG};
    if let Some(shell) = CFG.completions {
        generate_completion(shell);
        std::process::exit(0);
    }

    if let Err(error) = exec() {
        println!("{error}");
    }
}

#[cfg(test)]
mod test {}
