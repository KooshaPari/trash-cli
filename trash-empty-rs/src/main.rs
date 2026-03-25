use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus};

const PYTHON_BOOTSTRAP: &str = r#"import os
import sys
from trashcli.empty.main import main as main

sys.argv[0] = os.environ.get('TRASH_EMPTY_WRAPPER_NAME', 'trash-empty')
raise SystemExit(main())
"#;
const PYTHON_EXECUTABLES: [&str; 2] = ["python3", "python"];

fn find_interpreter() -> Option<String> {
    if let Ok(explicit) = env::var("TRASH_EMPTY_PYTHON_EXECUTABLE") {
        if is_python_interpreter(&explicit) {
            return Some(explicit);
        }
    }

    PYTHON_EXECUTABLES.iter().find_map(|candidate| {
        if is_python_interpreter(candidate) {
            Some((*candidate).to_string())
        } else {
            None
        }
    })
}

fn is_python_interpreter(candidate: &str) -> bool {
    Command::new(candidate)
        .arg("-c")
        .arg("pass")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_python_backend() -> std::io::Result<ExitStatus> {
    let mut args = env::args_os();
    let wrapper_arg = args.next().unwrap_or_else(|| "trash-empty".into());
    let wrapper_name = Path::new(&wrapper_arg)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("trash-empty");

    let python = find_interpreter().unwrap_or_else(|| "python3".to_string());
    let mut command = Command::new(python);
    let status = command
        .arg("-c")
        .arg(PYTHON_BOOTSTRAP)
        .env("TRASH_EMPTY_WRAPPER_NAME", wrapper_name)
        .args(args)
        .status()?;
    Ok(status)
}

fn main() {
    match run_python_backend() {
        Ok(status) => {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
            // Python exited by signal; propagate a generic failure code.
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("trash-empty: failed to invoke Python backend: {}", err);
            std::process::exit(1);
        }
    }
}
