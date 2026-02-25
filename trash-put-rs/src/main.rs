use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus};

const PYTHON_BOOTSTRAP: &str = "import os, sys\nfrom trashcli.put.main import main as main\n\nsys.argv[0] = os.environ.get('TRASH_PUT_WRAPPER_NAME', 'trash-put')\nraise SystemExit(main())";

const PYTHON_EXECUTABLES: [&str; 2] = ["python3", "python"];

fn find_interpreter() -> Option<&'static str> {
    PYTHON_EXECUTABLES.iter().find(|candidate| {
        Command::new(candidate)
            .arg("-c")
            .arg("pass")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }).copied()
}

fn run_python_backend() -> std::io::Result<ExitStatus> {
    let mut args = env::args_os();
    let wrapper_arg = args.next().unwrap_or_else(|| "trash-put".into());
    let wrapper_name = Path::new(&wrapper_arg)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("trash-put");

    let python = find_interpreter().unwrap_or("python3");
    let mut command = Command::new(python);
    let status = command
        .arg("-c")
        .arg(PYTHON_BOOTSTRAP)
        .env("TRASH_PUT_WRAPPER_NAME", wrapper_name)
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
            eprintln!("trash-put: failed to invoke Python backend: {}", err);
            std::process::exit(1);
        }
    }
}
