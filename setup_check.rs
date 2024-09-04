use log::{info, warn, error};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::user_management::UserType;
use crate::setup_project::ProjectSetup;
use crate::zk_utils::ZKSetup;

fn check_requirements() -> Result<Vec<String>, io::Error> {
    let requirements_path = "requirements.txt";
    let path = Path::new(requirements_path);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Requirements file not found: {}", requirements_path)));
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut missing = Vec::new();
    for line in reader.lines() {
        let requirement = line?.trim().to_string();
        if !requirement.is_empty() && !requirement.starts_with('#') {
            // In Rust, we don't have a direct equivalent to Python's pkg_resources.
            // You might want to implement a custom function to check if a package is installed.
            // For now, we'll just add all requirements to the missing list.
            missing.push(requirement);
        }
    }

    Ok(missing)
}

pub fn check_and_fix_setup(user_type: UserType, user_data: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking setup for user type: {:?}", user_type);

    // Check requirements
    let missing_packages = check_requirements()?;
    if !missing_packages.is_empty() {
        warn!("Missing packages: {}. Please install them.", missing_packages.join(", "));
        return Ok(());
    }

    // Create a ProjectSetup instance
    let mut project_setup = ProjectSetup::new(user_type, user_data.clone());

    // Perform setup checks
    if !project_setup.check_common_environment() {
        warn!("Common environment setup incomplete. Fixing...");
        project_setup.setup_common_environment()?;
    }

    // User-specific checks
    match user_type {
        UserType::Creator => {
            if !project_setup.check_creator_setup() {
                warn!("Creator-specific setup incomplete. Fixing...");
                project_setup.setup_creator_project()?;
            }
        },
        UserType::Developer => {
            if !project_setup.check_developer_setup() {
                warn!("Developer-specific setup incomplete. Fixing...");
                project_setup.setup_developer_project()?;
            }
        },
        UserType::Normal => {
            if !project_setup.check_normal_user_setup() {
                warn!("Normal user-specific setup incomplete. Fixing...");
                project_setup.setup_normal_user_project()?;
            }
        },
    }

    // Setup ZK environment
    let mut zk_setup = ZKSetup::new(user_type, user_data);
    if !zk_setup.check_zk_environment() {
        warn!("ZK environment setup incomplete. Fixing...");
        zk_setup.setup_zk_environment()?;
    }

    info!("Setup check and fix completed successfully");
    Ok(())
}
