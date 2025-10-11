use clap::Parser;
use ri::{
    add_script_interactive, detect_package_manager, get_package_json_scripts_with_commands,
    run_script,
};

#[derive(Parser)]
#[command(name = "rr")]
#[command(about = "Run package scripts")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Add a new script to package.json
    #[arg(short = 'a', long)]
    add_script: bool,

    /// List all scripts in package.json
    #[arg(short = 'l', long)]
    list_scripts: bool,

    /// Script name to run
    script: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle add script flag (doesn't need package manager detection)
    if cli.add_script {
        return add_script_interactive().await;
    }

    // Handle list scripts flag
    if cli.list_scripts {
        if !std::path::Path::new("package.json").exists() {
            std::process::exit(0);
        }

        match get_package_json_scripts_with_commands() {
            Ok(scripts) => {
                if scripts.is_empty() {
                    println!("No scripts found in package.json");
                } else {
                    println!("Available scripts:");
                    for (name, command) in scripts {
                        println!("  {}: {}", name, command);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading package.json: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Check if package.json exists for script operations
    if !std::path::Path::new("package.json").exists() {
        std::process::exit(0);
    }

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

    // Default to interactive mode if no script provided
    let interactive = cli.script.is_none();
    run_script(&package_manager, cli.script, interactive).await
}
