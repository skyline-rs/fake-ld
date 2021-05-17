use std::env;
use std::path::Path;
use std::process::Command;

const WINDOWS_PATH: &str = r#"C:\ProgramData\Chocolatey\bin\gcc.exe"#;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let cmd = if Path::new(WINDOWS_PATH).exists() {
        WINDOWS_PATH
    } else {
        "gcc"
    };

    let status = Command::new(cmd)
        .args(&args)
        .status()
        .unwrap();

    if let Some(x) = status.code() {
        std::process::exit(x);
    }
}
