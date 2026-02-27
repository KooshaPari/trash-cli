use std::env;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};

const PYTHON_EXECUTABLES: [&str; 2] = ["python3", "python"];

fn main() {
    let python = find_python_interpreter();
    let mut command = Command::new(python);

    let mut args: Vec<String> = env::args().collect();
    if !args.is_empty() {
        args.remove(0);
    }

    let status = command
        .arg("-m")
        .arg("trashcli.restore.main")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) => {
            let code = status.code().unwrap_or(1);
            exit(code);
        }
        Err(err) => {
            eprintln!("failed to start Python restore backend: {}", err);
            exit(1);
        }
    }
}

fn find_python_interpreter() -> PathBuf {
    if let Ok(py) = env::var("TRASH_RESTORE_PYTHON") {
        return Path::new(&py).to_path_buf();
    }

    for candidate in PYTHON_EXECUTABLES.iter() {
        if let Ok(py) = which::which(candidate) {
            return py;
        }
    }

    Path::new("python").to_path_buf()
}
