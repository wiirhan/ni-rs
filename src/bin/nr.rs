use clap::Parser;
use ni_rs::run_npm_script;
use std::process::exit;

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
