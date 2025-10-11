use clap::Parser;
use ri::{detect_package_manager, install_packages};

#[derive(Parser)]
#[command(name = "ri")]
#[command(about = "Rust package manager CLI inspired by @antfu/ni")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Packages to install
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
        eprintln!(
            "Package manager '{}' is not available",
            package_manager.command()
        );
        std::process::exit(1);
    }

    if cli.verbose {
        println!("Detected package manager: {}", package_manager.command());
    }

    install_packages(&package_manager, cli.packages).await
}
