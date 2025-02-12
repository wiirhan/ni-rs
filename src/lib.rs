use std::process::{Command, ExitStatus, Stdio};

pub fn run_npm_install() -> ExitStatus {
    Command::new("npm")
        .arg("install")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command")
}

pub fn run_npm_script(script: &str, args: &[String]) -> ExitStatus {
    let mut cmd = Command::new("npm");
    cmd.arg("run").arg(script);

    for arg in args {
        cmd.arg(arg);
    }

    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command")
}
