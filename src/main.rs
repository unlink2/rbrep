use rbrep::{config::generate_completion, exec, CFG};

fn main() {
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
