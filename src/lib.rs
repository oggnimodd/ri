pub mod commands;
pub mod config;
pub mod detection;
pub mod interactive;

pub use commands::{
    execute_command, install_packages, run_script, uninstall_packages, update_packages,
};
pub use detection::{detect_package_manager, is_package_manager_available, PackageManager};
pub use interactive::{
    add_script_interactive, get_package_json_scripts_with_commands, select_package_interactive,
    select_script_interactive,
};
