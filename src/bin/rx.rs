use clap::Parser;
use ri::{detect_package_manager, execute_command};

#[derive(Parser)]
#[command(name = "rx")]
#[command(about = "Execute package binaries")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Command to execute and its arguments
    #[arg(trailing_var_arg = true)]
    command_and_args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.command_and_args.is_empty() {
        eprintln!("Error: No command provided");
        std::process::exit(1);
    }

    let command = cli.command_and_args[0].clone();
    let args = cli.command_and_args[1..].to_vec();

    let current_dir = std::env::current_dir()?;
    let package_manager = detect_package_manager(&current_dir)
        .ok_or("No package manager detected in current directory")?;

    if !ri::is_package_manager_available(&package_manager) {
        eprintln!("Package manager '{}' is not available", package_manager.command());
        std::process::exit(1);
    }

    if cli.verbose {
        println!("Detected package manager: {}", package_manager.command());
    }

    execute_command(&package_manager, command, args).await
}
