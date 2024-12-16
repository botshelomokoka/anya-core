#!/usr/bin/env python3
"""
Unified installer for Anya Project.
Handles cross-platform installation and setup.
"""

import sys
import os
from pathlib import Path
import argparse
import logging
from typing import Optional

from lib.utils import (
    SystemInfo,
    DependencyManager,
    ConfigManager,
    ServiceManager,
    Logger
)

class AnyaInstaller:
    """Main installer class for Anya project."""
    
    def __init__(self, project_root: Optional[Path] = None):
        self.project_root = project_root or Path(__file__).parent.parent
        self.log_dir = self.project_root / 'logs'
        self.logger = Logger(self.log_dir)
        self.system_info = SystemInfo()
        self.dep_manager = DependencyManager()
        self.config_manager = ConfigManager(self.project_root)
        self.service_manager = ServiceManager(self.project_root)
    
    def check_system_requirements(self) -> bool:
        """Check if system meets minimum requirements."""
        specs = self.system_info.get_system_specs()
        
        min_requirements = {
            'cpu_cores': 2,
            'memory_gb': 4
        }
        
        meets_requirements = (
            specs['cpu_cores'] >= min_requirements['cpu_cores'] and
            specs['memory_gb'] >= min_requirements['memory_gb']
        )
        
        if not meets_requirements:
            logging.error("System does not meet minimum requirements:")
            logging.error(f"CPU Cores: {specs['cpu_cores']} (min: {min_requirements['cpu_cores']})")
            logging.error(f"Memory: {specs['memory_gb']}GB (min: {min_requirements['memory_gb']}GB)")
            return False
        
        return True
    
    def setup_development_environment(self):
        """Set up development environment."""
        logging.info("Setting up development environment...")
        
        # Install Rust if not present
        if not self.dep_manager.check_rust_installation():
            logging.info("Installing Rust...")
            self.dep_manager.install_rust()
        
        # Install system dependencies
        logging.info("Installing system dependencies...")
        self.dep_manager.install_system_dependencies()
        
        # Configure Git
        logging.info("Configuring Git...")
        self.config_manager.configure_git()
    
    def setup_project(self):
        """Set up project-specific configuration."""
        logging.info("Setting up project...")
        
        # Load existing config or create new one
        config = self.config_manager.load_config()
        
        # Update system information
        config['system'] = self.system_info.get_system_specs()
        
        # Save updated config
        self.config_manager.save_config(config)
        
        # Set up system service
        logging.info("Setting up system service...")
        self.service_manager.setup_service()
    
    def install(self):
        """Run full installation process."""
        logging.info("Starting Anya installation...")
        
        try:
            # Check system requirements
            if not self.check_system_requirements():
                sys.exit(1)
            
            # Setup development environment
            self.setup_development_environment()
            
            # Setup project
            self.setup_project()
            
            logging.info("Installation completed successfully!")
            
        except Exception as e:
            logging.error(f"Installation failed: {str(e)}")
            sys.exit(1)

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Anya Project Installer")
    parser.add_argument('--project-root', type=Path,
                       help='Project root directory')
    args = parser.parse_args()
    
    installer = AnyaInstaller(args.project_root)
    installer.install()

if __name__ == '__main__':
    main()
