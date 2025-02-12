use std::{
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    // 检测lock文件
    let package_manager = detect_package_manager().unwrap_or_else(|| {
        // 如果没有lock文件，提示用户选择
        prompt_select_package_manager()
    });

    // 执行安装命令
    let status = run_install_command(package_manager);

    // 处理执行结果
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}

/// 检测当前目录的lock文件
fn detect_package_manager() -> Option<&'static str> {
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
    ];

    for (file, manager) in lock_files {
        if Path::new(file).exists() {
            return Some(manager);
        }
    }

    None
}

/// 提示用户选择包管理器
fn prompt_select_package_manager() -> &'static str {
    let options = ["npm", "yarn", "pnpm", "bun"];

    // 使用dialoguer创建交互式选择列表
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("No lock file found. Please select a package manager:")
        .items(&options)
        .default(0) // 默认选中第一个选项
        .interact()
        .expect("Failed to select package manager");

    options[selection]
}

/// 执行安装命令
fn run_install_command(package_manager: &str) -> ExitStatus {
    let command = match package_manager {
        "npm" => "install",
        "yarn" => "install",
        "pnpm" => "install",
        "bun" => "install",
        _ => panic!("Unsupported package manager: {}", package_manager),
    };

    Command::new(package_manager)
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command")
}
