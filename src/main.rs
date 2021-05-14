use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .map(|x| {
            if x.starts_with("-Wl,") {
                x.split(',').skip(1).map(String::from).collect::<Vec<_>>().into_iter()
            } else if x == "-fuse-ld=lld" {
                vec![].into_iter()
            } else {
                vec![x].into_iter()
            }
        })
        .flatten()
        .collect();

    let status = Command::new("aarch64-none-elf-ld").args(&args).status().unwrap();

    if let Some(x) = status.code() {
        std::process::exit(x);
    }
}
