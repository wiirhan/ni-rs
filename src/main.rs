// src/main.rs
use clap::{Args, Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "nr")]
#[command(about = "npm command shortcut", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 执行 npm install
    Ni,
    /// 执行 npm run <script>
    Nr(NrArgs),
}

#[derive(Args)]
struct NrArgs {
    script: String,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ni => run_npm_install(),
        Commands::Nr(args) => run_npm_script(&args.script, &args.args),
    }
}

fn run_npm_install() {
    let status = Command::new("npm")
        .arg("install")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}

fn run_npm_script(script: &str, args: &[String]) {
    let mut cmd = Command::new("npm");
    cmd.arg("run").arg(script);

    for arg in args {
        cmd.arg(arg);
    }

    let status = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}
