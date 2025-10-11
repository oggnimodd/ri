use crate::detection::PackageManager;
use inquire::{Confirm, Select, Text};

pub async fn select_package_interactive() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let packages = select_packages_interactive().await?;
    Ok(packages)
}

pub async fn select_packages_interactive() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dependencies = get_package_dependencies()?;

    if dependencies.is_empty() {
        return Ok(vec![]); // Return empty list if no dependencies found
    }

    // Show a selectable list of dependencies
    match Select::new("Select packages to uninstall:", dependencies.clone())
        .without_help_message()
        .prompt()
    {
        Ok(selected) => Ok(vec![selected]),
        Err(inquire::error::InquireError::OperationCanceled) => Ok(vec![]), // Gracefully handle cancellation
        Err(e) => Err(e.into()),
    }
}

fn get_package_dependencies() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !std::path::Path::new("package.json").exists() {
        return Ok(vec![]);
    }

    let content = std::fs::read_to_string("package.json")?;
    let package_json: serde_json::Value = serde_json::from_str(&content)?;

    let mut dependencies = Vec::new();

    // Get dependencies
    if let Some(deps) = package_json.get("dependencies").and_then(|d| d.as_object()) {
        dependencies.extend(deps.keys().cloned());
    }

    // Get devDependencies
    if let Some(dev_deps) = package_json
        .get("devDependencies")
        .and_then(|d| d.as_object())
    {
        dependencies.extend(dev_deps.keys().cloned());
    }

    // Get peerDependencies
    if let Some(peer_deps) = package_json
        .get("peerDependencies")
        .and_then(|d| d.as_object())
    {
        dependencies.extend(peer_deps.keys().cloned());
    }

    // Remove duplicates and sort
    dependencies.sort();
    dependencies.dedup();

    Ok(dependencies)
}

pub async fn select_script_interactive(
    _pm: &PackageManager,
) -> Result<String, Box<dyn std::error::Error>> {
    select_package_json_script_interactive().await
}

async fn select_package_json_script_interactive() -> Result<String, Box<dyn std::error::Error>> {
    let scripts = get_package_json_scripts_with_commands()?;

    if scripts.is_empty() {
        std::process::exit(0);
    }

    // Show all scripts in a list for arrow key navigation
    let display_options: Vec<String> = scripts
        .iter()
        .map(|(name, command)| format!("{} -> {}", name, command))
        .collect();

    match Select::new("Select script:", display_options)
        .without_help_message()
        .prompt()
    {
        Ok(selected) => {
            // Find the corresponding script name from the selected display text
            for (name, command) in scripts.iter() {
                let display_text = format!("{} -> {}", name, command);
                if display_text == selected {
                    return Ok(name.clone());
                }
            }
            Err("Script selection failed".into())
        }
        Err(inquire::error::InquireError::OperationCanceled) => std::process::exit(0),
        Err(e) => Err(e.into()),
    }
}

pub fn get_package_json_scripts_with_commands(
) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("package.json")?;
    let package_json: serde_json::Value = serde_json::from_str(&content)?;

    if let Some(scripts) = package_json.get("scripts").and_then(|s| s.as_object()) {
        let mut result = std::collections::HashMap::new();
        for (name, command) in scripts {
            if let Some(command_str) = command.as_str() {
                result.insert(name.clone(), command_str.to_string());
            }
        }
        Ok(result)
    } else {
        Ok(std::collections::HashMap::new())
    }
}

pub async fn add_script_interactive() -> Result<(), Box<dyn std::error::Error>> {
    // Check if package.json exists
    if !std::path::Path::new("package.json").exists() {
        match Confirm::new("No package.json found. Create one?")
            .with_default(true)
            .prompt()
        {
            Ok(true) => {
                create_default_package_json()?;
                println!("Created package.json");
            }
            Ok(false) => {
                println!("Cancelled. No package.json created.");
                return Ok(());
            }
            Err(inquire::error::InquireError::OperationCanceled) => std::process::exit(0),
            Err(e) => return Err(e.into()),
        }
    }

    // Get script name and command
    let script_name = match Text::new("Script name:").prompt() {
        Ok(name) => name,
        Err(inquire::error::InquireError::OperationCanceled) => std::process::exit(0),
        Err(e) => return Err(e.into()),
    };

    let script_command = match Text::new("Script command:").prompt() {
        Ok(command) => command,
        Err(inquire::error::InquireError::OperationCanceled) => std::process::exit(0),
        Err(e) => return Err(e.into()),
    };

    // Read existing package.json
    let mut package_json = read_package_json()?;

    // Check if script already exists
    if let Some(scripts) = package_json
        .get_mut("scripts")
        .and_then(|s| s.as_object_mut())
    {
        if scripts.contains_key(&script_name) {
            match Confirm::new(&format!(
                "Script '{}' already exists. Overwrite?",
                script_name
            ))
            .with_default(false)
            .prompt()
            {
                Ok(true) => {
                    scripts.insert(
                        script_name.clone(),
                        serde_json::Value::String(script_command.clone()),
                    );
                }
                Ok(false) => {
                    println!("Cancelled. Script not modified.");
                    return Ok(());
                }
                Err(inquire::error::InquireError::OperationCanceled) => std::process::exit(0),
                Err(e) => return Err(e.into()),
            }
        } else {
            scripts.insert(
                script_name.clone(),
                serde_json::Value::String(script_command.clone()),
            );
        }
    } else {
        // Create scripts object if it doesn't exist
        package_json["scripts"] = serde_json::json!({
            script_name.clone(): script_command.clone()
        });
    }

    // Write back to package.json
    write_package_json(&package_json)?;
    println!("Added script '{}' -> '{}'", script_name, script_command);

    Ok(())
}

fn create_default_package_json() -> Result<(), Box<dyn std::error::Error>> {
    let default_package = serde_json::json!({
        "name": "my-project",
        "version": "1.0.0",
        "description": "",
        "main": "index.js",
        "scripts": {},
        "keywords": [],
        "author": "",
        "license": "ISC"
    });

    write_package_json(&default_package)
}

fn read_package_json() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("package.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn write_package_json(package_json: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(package_json)?;
    std::fs::write("package.json", content)?;
    Ok(())
}
