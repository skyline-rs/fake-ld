use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};

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

fn non_allowed_option(x: &str) -> bool {
    false
}

fn is_linker(args: &[String]) -> bool {
    args.iter().any(|x| x == "--eh-frame-hdr")
}

fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .filter_map(|arg| {
            if arg.starts_with("-init=") || arg.starts_with("-fini=") {
                Some(arg.split("=").map(String::from).collect::<Vec<_>>().into_iter())
            } else if arg.starts_with("-Wl,") {
                Some(
                    arg.split(',')
                        .skip(1)
                        .filter(|x| !non_allowed_option(*x))
                        .map(String::from)
                        .collect::<Vec<_>>()
                        .into_iter()
                )
            } else if !non_allowed_option(&arg) {
                Some(vec![arg].into_iter())
            } else {
                None
            }
        })
        .flatten()
        .collect();
    let is_linker = is_linker(&args);
    dbg!(&args);
    if (args.len() > 2 && args[0] == "-flavor" && args[1] == "gnu") || is_linker {
        let exit_code = Command::new(find_lld().expect("Could not locate ld.lld"))
            .args(args)
            .status()
            .unwrap()
            .code()
            .unwrap_or(0);
        std::process::exit(exit_code);
    } else {
        let exit_code = Command::new("zig")
            .arg("cc")
            .args(args)
            .status()
            .unwrap()
            .code()
            .unwrap_or(0);
        std::process::exit(exit_code);
    };
}
