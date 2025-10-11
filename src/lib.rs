pub mod detection;
pub mod commands;
pub mod interactive;
pub mod config;

pub use detection::{PackageManager, detect_package_manager, is_package_manager_available};
pub use commands::{install_packages, run_script, execute_command, update_packages, uninstall_packages};
pub use interactive::{select_package_interactive, select_script_interactive, add_script_interactive, get_package_json_scripts_with_commands};
