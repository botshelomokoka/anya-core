"""
Unified project configuration system for Anya.
Manages project-wide settings, dependencies, and environment configuration.
"""

import os
import sys
import json
import toml
import yaml
from pathlib import Path
from typing import Dict, Any, Optional, List
from dataclasses import dataclass
from .utils import SystemInfo, Logger

@dataclass
class ProjectPaths:
    """Project path configuration."""
    root: Path
    config: Path
    scripts: Path
    src: Path
    docs: Path
    tests: Path
    dependencies: Path
    enterprise: Path
    extensions: Path
    migrations: Path

class ProjectConfig:
    """Unified project configuration manager."""
    
    def __init__(self, project_root: Optional[Path] = None):
        self.project_root = project_root or Path(__file__).parent.parent.parent
        self.paths = self._initialize_paths()
        self.system_info = SystemInfo()
        self.logger = Logger(self.paths.root / 'logs')
        self._load_configs()
    
    def _initialize_paths(self) -> ProjectPaths:
        """Initialize project paths."""
        return ProjectPaths(
            root=self.project_root,
            config=self.project_root / 'config',
            scripts=self.project_root / 'scripts',
            src=self.project_root / 'src',
            docs=self.project_root / 'docs',
            tests=self.project_root / 'tests',
            dependencies=self.project_root / 'dependencies',
            enterprise=self.project_root / 'enterprise',
            extensions=self.project_root / 'anya-extensions',
            migrations=self.project_root / 'migrations'
        )
    
    def _load_configs(self):
        """Load all project configurations."""
        self.cargo_config = self._load_cargo_config()
        self.project_config = self._load_project_config()
        self.env_config = self._load_env_config()
    
    def _load_cargo_config(self) -> Dict[str, Any]:
        """Load Cargo.toml configuration."""
        cargo_path = self.paths.root / 'Cargo.toml'
        return toml.load(cargo_path)
    
    def _load_project_config(self) -> Dict[str, Any]:
        """Load project-specific configuration."""
        config_path = self.paths.config / 'project_config.yaml'
        if not config_path.exists():
            self._create_default_project_config(config_path)
        return yaml.safe_load(config_path.read_text())
    
    def _load_env_config(self) -> Dict[str, str]:
        """Load environment configuration."""
        env_path = self.paths.root / '.env'
        if not env_path.exists():
            self._create_default_env_config(env_path)
        return self._parse_env_file(env_path)
    
    def _create_default_project_config(self, config_path: Path):
        """Create default project configuration."""
        default_config = {
            'project': {
                'name': 'anya',
                'version': self.cargo_config.get('package', {}).get('version', '0.1.0'),
                'description': 'Decentralized AI Governance System'
            },
            'features': {
                'bitcoin_integration': True,
                'web5_support': True,
                'enterprise_features': True
            },
            'dependencies': {
                'rust': '>=1.70.0',
                'node': '>=18.0.0',
                'python': '>=3.8.0'
            },
            'system_requirements': {
                'min_cpu_cores': 2,
                'min_memory_gb': 4,
                'min_disk_space_gb': 50
            }
        }
        config_path.parent.mkdir(parents=True, exist_ok=True)
        yaml.dump(default_config, config_path.open('w'), default_flow_style=False)
    
    def _create_default_env_config(self, env_path: Path):
        """Create default environment configuration."""
        default_env = {
            'ANYA_ENV': 'development',
            'RUST_LOG': 'debug',
            'RUST_BACKTRACE': '1',
            'DATABASE_URL': 'postgresql://localhost/anya',
            'BITCOIN_NETWORK': 'testnet',
            'WEB5_ENDPOINT': 'http://localhost:8080',
            'ENABLE_ENTERPRISE': 'true'
        }
        
        with env_path.open('w') as f:
            for key, value in default_env.items():
                f.write(f'{key}={value}\n')
    
    def _parse_env_file(self, env_path: Path) -> Dict[str, str]:
        """Parse environment file."""
        env_vars = {}
        for line in env_path.read_text().splitlines():
            if line and not line.startswith('#'):
                key, value = line.split('=', 1)
                env_vars[key.strip()] = value.strip()
        return env_vars
    
    def get_dependency_info(self) -> Dict[str, str]:
        """Get project dependency information."""
        return {
            'rust': self.cargo_config.get('package', {}).get('rust-version', 'unknown'),
            'node': self.project_config.get('dependencies', {}).get('node', 'unknown'),
            'python': self.project_config.get('dependencies', {}).get('python', 'unknown')
        }
    
    def get_feature_flags(self) -> Dict[str, bool]:
        """Get enabled feature flags."""
        return self.project_config.get('features', {})
    
    def get_system_requirements(self) -> Dict[str, int]:
        """Get system requirements."""
        return self.project_config.get('system_requirements', {})
    
    def update_config(self, section: str, key: str, value: Any):
        """Update project configuration."""
        if section not in self.project_config:
            self.project_config[section] = {}
        self.project_config[section][key] = value
        
        config_path = self.paths.config / 'project_config.yaml'
        yaml.dump(self.project_config, config_path.open('w'), default_flow_style=False)
    
    def update_env(self, key: str, value: str):
        """Update environment variable."""
        self.env_config[key] = value
        env_path = self.paths.root / '.env'
        
        with env_path.open('w') as f:
            for k, v in self.env_config.items():
                f.write(f'{k}={v}\n')
    
    def validate_project_structure(self) -> List[str]:
        """Validate project directory structure."""
        issues = []
        required_dirs = [
            self.paths.config,
            self.paths.scripts,
            self.paths.src,
            self.paths.docs,
            self.paths.tests,
            self.paths.dependencies,
            self.paths.enterprise,
            self.paths.extensions,
            self.paths.migrations
        ]
        
        for dir_path in required_dirs:
            if not dir_path.exists():
                issues.append(f"Missing required directory: {dir_path.relative_to(self.paths.root)}")
        
        required_files = [
            self.paths.root / 'Cargo.toml',
            self.paths.root / '.env',
            self.paths.root / 'README.md',
            self.paths.config / 'project_config.yaml'
        ]
        
        for file_path in required_files:
            if not file_path.exists():
                issues.append(f"Missing required file: {file_path.relative_to(self.paths.root)}")
        
        return issues
    
    def generate_project_info(self) -> Dict[str, Any]:
        """Generate comprehensive project information."""
        return {
            'project': self.project_config.get('project', {}),
            'features': self.get_feature_flags(),
            'dependencies': self.get_dependency_info(),
            'system': self.system_info.get_system_specs(),
            'environment': self.env_config,
            'paths': {k: str(v) for k, v in self.paths.__dict__.items()}
        }
