"""Shared utilities for Anya installation and setup."""
import os
import sys
import platform
import subprocess
import logging
from typing import List, Dict, Optional, Union
from pathlib import Path
import json
import shutil

class SystemInfo:
    """System information and utilities."""
    
    @staticmethod
    def get_os() -> str:
        """Get current operating system."""
        if sys.platform.startswith('win'):
            return 'windows'
        elif sys.platform.startswith('darwin'):
            return 'macos'
        elif sys.platform.startswith('linux'):
            return 'linux'
        else:
            raise OSError(f"Unsupported operating system: {sys.platform}")

    @staticmethod
    def get_package_manager() -> Optional[str]:
        """Detect system package manager."""
        os_type = SystemInfo.get_os()
        
        if os_type == 'windows':
            return 'winget' if shutil.which('winget') else None
        elif os_type == 'macos':
            return 'brew' if shutil.which('brew') else None
        elif os_type == 'linux':
            pkg_managers = {
                'apt-get': 'debian',
                'dnf': 'fedora',
                'yum': 'centos',
                'pacman': 'arch'
            }
            return next((pm for pm, _ in pkg_managers.items() 
                       if shutil.which(pm)), None)
        return None

    @staticmethod
    def get_system_specs() -> Dict[str, Union[str, int]]:
        """Get system specifications."""
        import psutil
        
        return {
            'os': SystemInfo.get_os(),
            'cpu_cores': os.cpu_count(),
            'memory_gb': round(psutil.virtual_memory().total / (1024**3)),
            'architecture': platform.machine(),
            'python_version': platform.python_version(),
        }

class DependencyManager:
    """Manage system and project dependencies."""
    
    @staticmethod
    def check_rust_installation() -> bool:
        """Check if Rust is installed."""
        try:
            subprocess.run(['rustc', '--version'], 
                         stdout=subprocess.PIPE, 
                         stderr=subprocess.PIPE)
            return True
        except FileNotFoundError:
            return False

    @staticmethod
    def install_rust():
        """Install Rust using rustup."""
        os_type = SystemInfo.get_os()
        if os_type == 'windows':
            subprocess.run(['powershell', '-Command',
                          'Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe'])
            subprocess.run(['rustup-init.exe', '-y'])
        else:
            subprocess.run(['curl', '--proto', '=https', '--tlsv1.2', '-sSf',
                          'https://sh.rustup.rs', '|', 'sh', '-s', '--', '-y'])

    @staticmethod
    def install_system_dependencies():
        """Install required system dependencies."""
        pkg_manager = SystemInfo.get_package_manager()
        if not pkg_manager:
            raise EnvironmentError("No supported package manager found")
        
        dependencies = {
            'winget': [
                'Microsoft.VisualStudioCode',
                'Git.Git',
                'OpenSSL.OpenSSL',
                'Microsoft.PowerShell'
            ],
            'brew': [
                'openssl',
                'pkg-config',
                'cmake',
                'llvm',
                'node',
                'git'
            ],
            'apt-get': [
                'build-essential',
                'pkg-config',
                'libssl-dev',
                'cmake',
                'llvm',
                'clang',
                'git'
            ]
        }
        
        if pkg_manager in dependencies:
            for dep in dependencies[pkg_manager]:
                if pkg_manager == 'winget':
                    subprocess.run(['winget', 'install', '-e', '--id', dep])
                elif pkg_manager == 'brew':
                    subprocess.run(['brew', 'install', dep])
                elif pkg_manager == 'apt-get':
                    subprocess.run(['sudo', 'apt-get', 'install', '-y', dep])

class ConfigManager:
    """Manage system and project configuration."""
    
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.config_file = project_root / 'config' / 'system_config.json'
        self.config_file.parent.mkdir(parents=True, exist_ok=True)
    
    def load_config(self) -> Dict:
        """Load system configuration."""
        if self.config_file.exists():
            with open(self.config_file) as f:
                return json.load(f)
        return {}
    
    def save_config(self, config: Dict):
        """Save system configuration."""
        with open(self.config_file, 'w') as f:
            json.dump(config, f, indent=4)
    
    def configure_git(self):
        """Configure Git settings."""
        os_type = SystemInfo.get_os()
        
        # Common Git configurations
        git_config = {
            'core.autocrlf': 'true' if os_type == 'windows' else 'input',
            'core.longpaths': 'true',
            'credential.helper': ('manager-core' if os_type == 'windows'
                                else 'osxkeychain' if os_type == 'macos'
                                else 'cache --timeout=3600'),
            'init.defaultBranch': 'main',
            'pull.rebase': 'false'
        }
        
        for key, value in git_config.items():
            subprocess.run(['git', 'config', '--global', key, value])

class ServiceManager:
    """Manage system services."""
    
    def __init__(self, project_root: Path):
        self.project_root = project_root
    
    def setup_service(self):
        """Set up system service based on OS."""
        os_type = SystemInfo.get_os()
        
        if os_type == 'windows':
            self._setup_windows_service()
        elif os_type == 'linux':
            self._setup_systemd_service()
        elif os_type == 'macos':
            self._setup_launchd_service()
    
    def _setup_windows_service(self):
        """Set up Windows service."""
        service_path = self.project_root / 'scripts' / 'services' / 'anya_service.ps1'
        subprocess.run(['powershell', '-File', str(service_path)])
    
    def _setup_systemd_service(self):
        """Set up systemd service."""
        service_path = Path('/etc/systemd/system/anya.service')
        service_content = f'''[Unit]
Description=Anya Web5 Service
After=network.target

[Service]
Type=simple
User={os.getenv('USER')}
WorkingDirectory={self.project_root}
ExecStart={os.path.expanduser('~')}/.cargo/bin/cargo run --release
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
'''
        with open(service_path, 'w') as f:
            f.write(service_content)
        
        subprocess.run(['systemctl', 'daemon-reload'])
        subprocess.run(['systemctl', 'enable', 'anya.service'])
    
    def _setup_launchd_service(self):
        """Set up launchd service."""
        plist_path = Path(f'~/Library/LaunchAgents/ai.anya.service.plist'
                         ).expanduser()
        plist_content = f'''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ai.anya.service</string>
    <key>ProgramArguments</key>
    <array>
        <string>{os.path.expanduser('~')}/.cargo/bin/cargo</string>
        <string>run</string>
        <string>--release</string>
    </array>
    <key>WorkingDirectory</key>
    <string>{self.project_root}</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>~/Library/Logs/anya.log</string>
    <key>StandardErrorPath</key>
    <string>~/Library/Logs/anya.error.log</string>
</dict>
</plist>
'''
        with open(plist_path, 'w') as f:
            f.write(plist_content)
        
        subprocess.run(['launchctl', 'load', '-w', str(plist_path)])

class Logger:
    """Unified logging system."""
    
    def __init__(self, log_dir: Path):
        self.log_dir = log_dir
        self.log_dir.mkdir(parents=True, exist_ok=True)
        self.setup_logging()
    
    def setup_logging(self):
        """Configure logging system."""
        log_file = self.log_dir / 'anya_install.log'
        
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s [%(levelname)s] %(message)s',
            handlers=[
                logging.FileHandler(log_file),
                logging.StreamHandler(sys.stdout)
            ]
        )
