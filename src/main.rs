use rbrep::exec;

fn main() {
    if let Err(error) = exec() {
        println!("{error}");
    }
}

#[cfg(test)]
mod test {}
