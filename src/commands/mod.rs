use std::process::Command;
use crate::detection::PackageManager;
use crate::interactive::{select_package_interactive, select_script_interactive};

pub async fn install_packages(
    pm: &PackageManager,
    packages: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let packages_to_install = if packages.is_empty() {
        vec![] // Install all dependencies
    } else {
        packages
    };

    let mut cmd = Command::new(pm.command());
    cmd.arg(pm.install_command());

    if !packages_to_install.is_empty() {
        cmd.args(&packages_to_install);
    }

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if status.success() {
        println!("Installation completed successfully");
    } else {
        eprintln!("Installation failed");
    }

    Ok(())
}

pub async fn run_script(
    pm: &PackageManager,
    script_name: Option<String>,
    interactive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let script = if let Some(name) = script_name {
        name
    } else if interactive {
        select_script_interactive(pm).await?
    } else {
        return Err("No script name provided".into());
    };

    // Check if script exists before trying to run it
    if !script_exists(&script, pm)? {
        std::process::exit(0);
    }

    let mut cmd = Command::new(pm.command());
    cmd.arg(pm.run_command());
    cmd.arg(&script);

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if status.success() {
        println!("Script completed successfully");
    } else {
        eprintln!("Script failed");
    }

    Ok(())
}

pub async fn execute_command(
    pm: &PackageManager,
    command: String,
    args: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(pm.command());
    cmd.arg(pm.exec_command());

    cmd.arg(&command);

    if !args.is_empty() {
        cmd.args(&args);
    }

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if status.success() {
        println!("Command completed successfully");
    } else {
        eprintln!("Command failed");
    }

    Ok(())
}

pub async fn update_packages(
    pm: &PackageManager,
    packages: Vec<String>,
    interactive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let packages_to_update = if interactive && packages.is_empty() {
        select_package_interactive().await?
    } else {
        packages
    };

    let mut cmd = Command::new(pm.command());
    cmd.arg(pm.update_command());

    if !packages_to_update.is_empty() {
        cmd.args(&packages_to_update);
    }

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if status.success() {
        println!("Update completed successfully");
    } else {
        eprintln!("Update failed");
    }

    Ok(())
}

pub async fn uninstall_packages(
    pm: &PackageManager,
    packages: Vec<String>,
    interactive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let packages_to_uninstall = if interactive && packages.is_empty() {
        select_package_interactive().await?
    } else if packages.is_empty() {
        return Err("No packages specified for uninstallation".into());
    } else {
        packages
    };

    // If no packages to uninstall (e.g., no dependencies found), exit gracefully
    if packages_to_uninstall.is_empty() {
        println!("No packages to uninstall.");
        return Ok(());
    }

    let mut cmd = Command::new(pm.command());
    
    match pm {
        PackageManager::Npm => {
            cmd.arg("uninstall");
            cmd.args(&packages_to_uninstall);
        }
        PackageManager::Yarn => {
            cmd.arg("remove");
            cmd.args(&packages_to_uninstall);
        }
        PackageManager::Pnpm => {
            cmd.arg("remove");
            cmd.args(&packages_to_uninstall);
        }
        PackageManager::Bun => {
            cmd.arg("remove");
            cmd.args(&packages_to_uninstall);
        }
    }

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if status.success() {
        println!("Uninstallation completed successfully");
    } else {
        eprintln!("Uninstallation failed");
    }

    Ok(())
}

fn script_exists(script_name: &str, _pm: &PackageManager) -> Result<bool, Box<dyn std::error::Error>> {
    let scripts = get_package_json_scripts()?;
    Ok(scripts.contains(&script_name.to_string()))
}

fn get_package_json_scripts() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("package.json")?;
    let package_json: serde_json::Value = serde_json::from_str(&content)?;
    
    if let Some(scripts) = package_json.get("scripts").and_then(|s| s.as_object()) {
        Ok(scripts.keys().cloned().collect())
    } else {
        Ok(vec![])
    }
}
