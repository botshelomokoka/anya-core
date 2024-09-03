import logging
import os
from typing import Dict, Any
from user_management import UserType
from setup_project import ProjectSetup
import pkg_resources

def check_requirements():
    """Check if all packages in requirements.txt are installed."""
    requirements_path = 'requirements.txt'
    if not os.path.exists(requirements_path):
        raise FileNotFoundError(f"Requirements file not found: {requirements_path}")

    with open(requirements_path, 'r') as f:
        requirements = [line.strip() for line in f if line.strip() and not line.startswith('#')]

    missing = []
    for requirement in requirements:
        try:
            pkg_resources.require(requirement)
        except pkg_resources.DistributionNotFound:
            missing.append(requirement)

    return missing

def check_and_fix_setup(user_type: str, user_data: Dict[str, Any]) -> None:
    """
    Check the project setup and fix any issues if necessary.
    
    Args:
        user_type (str): The type of user (e.g., 'CREATOR', 'DEVELOPER', 'NORMAL')
        user_data (Dict[str, Any]): Dictionary containing user-specific data
    """
    logger = logging.getLogger(__name__)
    logger.info(f"Checking setup for user type: {user_type}")

    try:
        # Check requirements
        missing_packages = check_requirements()
        if missing_packages:
            logger.warning(f"Missing packages: {', '.join(missing_packages)}. Please install them.")
            return

        # Create a ProjectSetup instance
        project_setup = ProjectSetup(user_type, user_data)

        # Perform setup checks
        if not project_setup.check_common_environment():
            logger.warning("Common environment setup incomplete. Fixing...")
            project_setup.setup_common_environment()

        # User-specific checks
        if user_type == UserType.CREATOR:
            if not project_setup.check_creator_setup():
                logger.warning("Creator-specific setup incomplete. Fixing...")
                project_setup.setup_creator_project()
        elif user_type == UserType.DEVELOPER:
            if not project_setup.check_developer_setup():
                logger.warning("Developer-specific setup incomplete. Fixing...")
                project_setup.setup_developer_project()
        elif user_type == UserType.NORMAL:
            if not project_setup.check_normal_user_setup():
                logger.warning("Normal user-specific setup incomplete. Fixing...")
                project_setup.setup_normal_user_project()
        else:
            logger.error(f"Unknown user type: {user_type}")
            return

        logger.info("Setup check and fix completed successfully")
    except Exception as e:
        logger.error(f"An error occurred during setup check and fix: {str(e)}")
