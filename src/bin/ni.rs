use ni_rs::run_npm_install;
use std::process::exit;

fn main() {
    let status = run_npm_install();

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        exit(1);
    }
}
