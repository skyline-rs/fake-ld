use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let status = Command::new("gcc")
        .args(&args)
        .status()
        .unwrap();

    if let Some(x) = status.code() {
        std::process::exit(x);
    }
}
