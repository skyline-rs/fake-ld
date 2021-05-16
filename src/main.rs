use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn exists(command: &str) -> bool {
    Command::new(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn llvm_path() -> Option<PathBuf> {
    let brew_output = Command::new("brew")
        .args(&["--prefix", "llvm"])
        .output()
        .ok()?;

    if brew_output.status.success() {
        Some(Path::new(std::str::from_utf8(&brew_output.stdout).ok()?.trim()).join("bin"))
    } else {
        None
    }
}

fn brew_path() -> Option<PathBuf> {
    let path = llvm_path()?.join("ld.lld");

    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn find_lld() -> Option<String> {
    if exists("ld.lld") {
        Some("ld.lld".to_string())
    } else if Path::new("/usr/bin/ld.lld").exists() {
        Some("/usr/bin/ld.lld".to_string())
    } else if Path::new("/usr/bin/ld.lld-10").exists() {
        Some("/usr/bin/ld.lld-10".to_string())
    } else if Path::new(r#"C:\Program Files\LLVM\bin\ld.lld.exe"#).exists() {
        Some(r#"C:\Program Files\LLVM\bin\ld.lld.exe"#.to_string())
    } else if let Some(path) = brew_path() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .map(|x| {
            dbg!(&x);
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

    let lld = dbg!(find_lld()).expect("ld.lld not found");

    let status = Command::new(&lld).args(&args).status().unwrap();

    if let Some(x) = status.code() {
        std::process::exit(x);
    }
}
