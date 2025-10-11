use clap::Parser;
use ri::{detect_package_manager, update_packages};

#[derive(Parser)]
#[command(name = "ru")]
#[command(about = "Update packages")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Packages to update
    #[arg()]
    packages: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

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

    // Default to interactive mode if no packages provided
    let interactive = cli.packages.is_empty();
    update_packages(&package_manager, cli.packages, interactive).await
}
