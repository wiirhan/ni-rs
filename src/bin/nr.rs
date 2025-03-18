use clap::Parser;
use ni_rs::run_npm_script;
use std::process::exit;

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_test_env() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("package.json"),
            r#"{
                "scripts": {
                    "test": "echo 'test script'",
                    "test-watch": "echo 'test watch script'"
                }
            }"#,
        )
        .unwrap();
        dir
    }

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("nr").unwrap();
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("npm run shortcut"));
    }

    #[test]
    fn test_run_script() {
        let temp_dir = setup_test_env();
        let mut cmd = Command::cargo_bin("nr").unwrap();
        cmd.current_dir(temp_dir.path())
            .arg("test")
            .assert()
            .success()
            .stdout(predicate::str::contains("test script"));
    }

    #[test]
    fn test_run_script_with_args() {
        let temp_dir = setup_test_env();
        let mut cmd = Command::cargo_bin("nr").unwrap();
        cmd.current_dir(temp_dir.path())
            .arg("test-watch")
            .arg("--")
            .arg("--watch")
            .assert()
            .success()
            .stdout(predicate::str::contains("test watch script"));
    }
}

#[derive(Parser)]
#[command(
    name = "nr",
    about = "npm run shortcut",
    version = "1.0",
    trailing_var_arg = true
)]
struct Cli {
    /// 要运行的脚本名称
    script: String,

    /// 传递给脚本的额外参数
    #[arg(allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let status = run_npm_script(&cli.script, &cli.args);

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        exit(1);
    }
}
