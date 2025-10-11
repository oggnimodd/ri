use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn command(&self) -> &str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }

    pub fn install_command(&self) -> &str {
        match self {
            PackageManager::Npm => "install",
            PackageManager::Yarn => "add",
            PackageManager::Pnpm => "add",
            PackageManager::Bun => "install",
        }
    }

    pub fn run_command(&self) -> &str {
        match self {
            PackageManager::Npm => "run",
            PackageManager::Yarn => "run",
            PackageManager::Pnpm => "run",
            PackageManager::Bun => "run",
        }
    }

    pub fn exec_command(&self) -> &str {
        match self {
            PackageManager::Npm => "exec",
            PackageManager::Yarn => "exec",
            PackageManager::Pnpm => "exec",
            PackageManager::Bun => "x",
        }
    }

    pub fn update_command(&self) -> &str {
        match self {
            PackageManager::Npm => "update",
            PackageManager::Yarn => "upgrade",
            PackageManager::Pnpm => "update",
            PackageManager::Bun => "update",
        }
    }
}

pub fn detect_package_manager(dir: &Path) -> Option<PackageManager> {
    // Check for lock files first (highest priority)
    if dir.join("bun.lock").exists() || dir.join("bun.lockb").exists() {
        return Some(PackageManager::Bun);
    }
    if dir.join("pnpm-lock.yaml").exists() {
        return Some(PackageManager::Pnpm);
    }
    if dir.join("yarn.lock").exists() {
        return Some(PackageManager::Yarn);
    }
    if dir.join("package-lock.json").exists() {
        return Some(PackageManager::Npm);
    }

    // Check for package.json only (ignore Cargo files)
    if dir.join("package.json").exists() {
        return Some(PackageManager::Bun); // Default to bun for package.json
    }

    None
}

pub fn is_package_manager_available(pm: &PackageManager) -> bool {
    which::which(pm.command()).is_ok()
}
