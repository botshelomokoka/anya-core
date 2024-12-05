#!/usr/bin/env python3
"""
Anya Core Sensitive Data Management Setup Script
This script helps users set up and manage sensitive data for Anya Core installations.
"""

import os
import json
import yaml
import getpass
import argparse
from pathlib import Path
import secrets
import string
from datetime import datetime

class AnyaSensitiveDataManager:
    def __init__(self):
        self.config_dir = Path.home() / '.anya'
        self.config_file = self.config_dir / 'sensitive_config.yml'
        self.env_file = Path('.env')
        self.secrets_file = self.config_dir / 'secrets.yml'
        self.setup_config_dir()

    def setup_config_dir(self):
        """Create configuration directory if it doesn't exist."""
        self.config_dir.mkdir(parents=True, exist_ok=True)

    def generate_secret(self, length=32):
        """Generate a secure random secret."""
        alphabet = string.ascii_letters + string.digits + string.punctuation
        return ''.join(secrets.choice(alphabet) for _ in range(length))

    def get_user_input(self, prompt, secret=False, default=None):
        """Get user input with optional default value."""
        if default:
            prompt = f"{prompt} [{default}]: "
        else:
            prompt = f"{prompt}: "
        
        if secret:
            value = getpass.getpass(prompt)
        else:
            value = input(prompt)
        
        return value if value else default

    def setup_new_installation(self):
        """Set up a new Anya Core installation."""
        print("\n=== Anya Core Sensitive Data Setup ===\n")
        
        config = {
            'installation': {
                'name': self.get_user_input("Installation name", default="anya-core"),
                'environment': self.get_user_input(
                    "Environment (development/staging/production)",
                    default="development"
                )
            },
            'database': {
                'host': self.get_user_input("Database host", default="localhost"),
                'port': self.get_user_input("Database port", default="5432"),
                'name': self.get_user_input("Database name", default="anya"),
                'user': self.get_user_input("Database user", default="anya_user"),
                'password': self.get_user_input("Database password", secret=True)
            },
            'api': {
                'key': self.generate_secret(32),
                'secret': self.generate_secret(32)
            },
            'security': {
                'secret_key': self.generate_secret(50),
                'allowed_hosts': self.get_user_input(
                    "Allowed hosts (comma-separated)",
                    default="localhost,127.0.0.1"
                ).split(',')
            },
            'email': {
                'host': self.get_user_input("SMTP host", default="smtp.gmail.com"),
                'port': self.get_user_input("SMTP port", default="587"),
                'user': self.get_user_input("SMTP user"),
                'password': self.get_user_input("SMTP password", secret=True)
            },
            'clustering': {
                'enabled': self.get_user_input("Enable clustering? (y/N)", default="n").lower() == 'y',
                'mode': self.get_user_input(
                    "Cluster mode (active-passive)",
                    default="active-passive"
                ) if self.get_user_input("Enable clustering? (y/N)", default="n").lower() == 'y' else None,
                'nodes': [],
                'heartbeat_interval': 5,
                'failover_timeout': 30,
                'health_check': {
                    'enabled': False,
                    'interval': 30,
                    'timeout': 5,
                    'path': '/health'
                },
                'monitoring': {
                    'enabled': False,
                    'metrics': {
                        'enabled': False,
                        'endpoint': None,
                        'interval': 15
                    },
                    'logging': {
                        'enabled': False,
                        'endpoint': None,
                        'level': 'info'
                    }
                },
                'network': {
                    'encryption': {
                        'enabled': False,
                        'type': 'tls',
                        'cert_path': None,
                        'key_path': None
                    }
                }
            }
        }

        # Update clustering configuration
        if config['clustering']['enabled']:
            config['clustering'].update({
                'nodes': self.get_user_input(
                    "Cluster nodes (comma-separated hostnames/IPs)",
                    default="localhost:9000"
                ).split(','),
                'heartbeat_interval': int(self.get_user_input(
                    "Heartbeat interval in seconds",
                    default="5"
                )),
                'failover_timeout': int(self.get_user_input(
                    "Failover timeout in seconds",
                    default="30"
                ))
            })

            # Configure health checks
            if self.get_user_input("Enable health checks? (y/N)", default="n").lower() == 'y':
                config['clustering']['health_check'].update({
                    'enabled': True,
                    'interval': int(self.get_user_input(
                        "Health check interval (seconds)",
                        default="30"
                    )),
                    'timeout': int(self.get_user_input(
                        "Health check timeout (seconds)",
                        default="5"
                    )),
                    'path': self.get_user_input(
                        "Health check path",
                        default="/health"
                    )
                })

            # Configure basic monitoring
            if self.get_user_input("Enable monitoring? (y/N)", default="n").lower() == 'y':
                config['clustering']['monitoring'].update({
                    'enabled': True,
                    'metrics': {
                        'enabled': True,
                        'endpoint': self.get_user_input(
                            "Metrics endpoint",
                            default="http://localhost:9090"
                        ),
                        'interval': int(self.get_user_input(
                            "Metrics collection interval (seconds)",
                            default="15"
                        ))
                    },
                    'logging': {
                        'enabled': True,
                        'endpoint': self.get_user_input(
                            "Logging endpoint",
                            default="http://localhost:9200"
                        ),
                        'level': self.get_user_input(
                            "Log level (debug/info/warn/error)",
                            default="info"
                        )
                    }
                })

            # Configure network security
            if self.get_user_input("Enable network encryption? (y/N)", default="n").lower() == 'y':
                config['clustering']['network']['encryption'].update({
                    'enabled': True,
                    'type': self.get_user_input(
                        "Encryption type (tls)",
                        default="tls"
                    ),
                    'cert_path': self.get_user_input(
                        "Certificate path",
                        default="/etc/anya/certs/server.crt"
                    ),
                    'key_path': self.get_user_input(
                        "Key path",
                        default="/etc/anya/certs/server.key"
                    )
                })

        # Save configuration
        self.save_config(config)
        self.generate_env_file(config)
        print("\nConfiguration completed successfully!")

    def update_existing_config(self):
        """Update existing configuration."""
        if not self.config_file.exists():
            print("No existing configuration found. Running new installation setup...")
            return self.setup_new_installation()

        with open(self.config_file, 'r') as f:
            config = yaml.safe_load(f)

        print("\n=== Update Anya Core Configuration ===\n")
        print("Enter new values or press Enter to keep existing ones.\n")

        # Update database configuration
        print("\nDatabase Configuration:")
        config['database']['host'] = self.get_user_input(
            "Database host", default=config['database']['host']
        )
        config['database']['password'] = self.get_user_input(
            "Database password", secret=True, default=config['database']['password']
        )

        # Update API credentials
        print("\nAPI Configuration:")
        if self.get_user_input("Generate new API credentials? (y/N)", default="n").lower() == 'y':
            config['api']['key'] = self.generate_secret(32)
            config['api']['secret'] = self.generate_secret(32)

        # Update security settings
        print("\nSecurity Configuration:")
        if self.get_user_input("Generate new secret key? (y/N)", default="n").lower() == 'y':
            config['security']['secret_key'] = self.generate_secret(50)

        # Update email configuration
        print("\nEmail Configuration:")
        config['email']['password'] = self.get_user_input(
            "SMTP password", secret=True, default=config['email']['password']
        )

        # Update clustering configuration
        if config['clustering']['enabled']:
            config['clustering'].update({
                'nodes': self.get_user_input(
                    "Cluster nodes (comma-separated hostnames/IPs)",
                    default="localhost:9000"
                ).split(','),
                'heartbeat_interval': int(self.get_user_input(
                    "Heartbeat interval in seconds",
                    default="5"
                )),
                'failover_timeout': int(self.get_user_input(
                    "Failover timeout in seconds",
                    default="30"
                ))
            })

            # Configure health checks
            if self.get_user_input("Enable health checks? (y/N)", default="n").lower() == 'y':
                config['clustering']['health_check'].update({
                    'enabled': True,
                    'interval': int(self.get_user_input(
                        "Health check interval (seconds)",
                        default="30"
                    )),
                    'timeout': int(self.get_user_input(
                        "Health check timeout (seconds)",
                        default="5"
                    )),
                    'path': self.get_user_input(
                        "Health check path",
                        default="/health"
                    )
                })

            # Configure basic monitoring
            if self.get_user_input("Enable monitoring? (y/N)", default="n").lower() == 'y':
                config['clustering']['monitoring'].update({
                    'enabled': True,
                    'metrics': {
                        'enabled': True,
                        'endpoint': self.get_user_input(
                            "Metrics endpoint",
                            default="http://localhost:9090"
                        ),
                        'interval': int(self.get_user_input(
                            "Metrics collection interval (seconds)",
                            default="15"
                        ))
                    },
                    'logging': {
                        'enabled': True,
                        'endpoint': self.get_user_input(
                            "Logging endpoint",
                            default="http://localhost:9200"
                        ),
                        'level': self.get_user_input(
                            "Log level (debug/info/warn/error)",
                            default="info"
                        )
                    }
                })

            # Configure network security
            if self.get_user_input("Enable network encryption? (y/N)", default="n").lower() == 'y':
                config['clustering']['network']['encryption'].update({
                    'enabled': True,
                    'type': self.get_user_input(
                        "Encryption type (tls)",
                        default="tls"
                    ),
                    'cert_path': self.get_user_input(
                        "Certificate path",
                        default="/etc/anya/certs/server.crt"
                    ),
                    'key_path': self.get_user_input(
                        "Key path",
                        default="/etc/anya/certs/server.key"
                    )
                })

        # Save updated configuration
        self.save_config(config)
        self.generate_env_file(config)
        print("\nConfiguration updated successfully!")

    def save_config(self, config):
        """Save configuration to file."""
        # Save sensitive configuration
        with open(self.secrets_file, 'w') as f:
            yaml.dump(config, f, default_flow_style=False)

        # Save non-sensitive configuration
        safe_config = {
            'installation': config['installation'],
            'database': {
                'host': config['database']['host'],
                'port': config['database']['port'],
                'name': config['database']['name'],
                'user': config['database']['user']
            },
            'security': {
                'allowed_hosts': config['security']['allowed_hosts']
            },
            'email': {
                'host': config['email']['host'],
                'port': config['email']['port'],
                'user': config['email']['user']
            },
            'clustering': {
                'enabled': config['clustering']['enabled'],
                'mode': config['clustering']['mode'],
                'nodes': config['clustering']['nodes'],
                'heartbeat_interval': config['clustering']['heartbeat_interval'],
                'failover_timeout': config['clustering']['failover_timeout'],
                'health_check': config['clustering']['health_check'],
                'monitoring': config['clustering']['monitoring'],
                'network': config['clustering']['network']
            }
        }
        with open(self.config_file, 'w') as f:
            yaml.dump(safe_config, f, default_flow_style=False)

    def generate_env_file(self, config):
        """Generate .env file from configuration."""
        env_content = f"""# Generated by Anya Core Setup on {datetime.now().isoformat()}
ANYA_ENV={config['installation']['environment']}

# Database Configuration
ANYA_DB_HOST={config['database']['host']}
ANYA_DB_PORT={config['database']['port']}
ANYA_DB_NAME={config['database']['name']}
ANYA_DB_USER={config['database']['user']}
ANYA_DB_PASSWORD={config['database']['password']}

# API Configuration
ANYA_API_KEY={config['api']['key']}
ANYA_API_SECRET={config['api']['secret']}

# Security Configuration
ANYA_SECRET_KEY={config['security']['secret_key']}
ANYA_ALLOWED_HOSTS={','.join(config['security']['allowed_hosts'])}

# Email Configuration
ANYA_SMTP_HOST={config['email']['host']}
ANYA_SMTP_PORT={config['email']['port']}
ANYA_SMTP_USER={config['email']['user']}
ANYA_SMTP_PASSWORD={config['email']['password']}

# Clustering Configuration
ANYA_CLUSTER_ENABLED={str(config['clustering']['enabled']).lower()}
ANYA_CLUSTER_MODE={config['clustering']['mode'] or ''}
ANYA_CLUSTER_NODES={','.join(config['clustering']['nodes'])}
ANYA_CLUSTER_HEARTBEAT_INTERVAL={config['clustering']['heartbeat_interval']}
ANYA_CLUSTER_FAILOVER_TIMEOUT={config['clustering']['failover_timeout']}

# Health Check Configuration
ANYA_CLUSTER_HEALTH_CHECK_ENABLED={str(config['clustering']['health_check']['enabled']).lower()}
ANYA_CLUSTER_HEALTH_CHECK_INTERVAL={config['clustering']['health_check']['interval']}
ANYA_CLUSTER_HEALTH_CHECK_TIMEOUT={config['clustering']['health_check']['timeout']}
ANYA_CLUSTER_HEALTH_CHECK_PATH={config['clustering']['health_check']['path']}

# Monitoring Configuration
ANYA_CLUSTER_MONITORING_ENABLED={str(config['clustering']['monitoring']['enabled']).lower()}
ANYA_CLUSTER_MONITORING_METRICS_ENABLED={str(config['clustering']['monitoring']['metrics']['enabled']).lower()}
ANYA_CLUSTER_MONITORING_METRICS_ENDPOINT={config['clustering']['monitoring']['metrics']['endpoint']}
ANYA_CLUSTER_MONITORING_METRICS_INTERVAL={config['clustering']['monitoring']['metrics']['interval']}
ANYA_CLUSTER_MONITORING_LOGGING_ENABLED={str(config['clustering']['monitoring']['logging']['enabled']).lower()}
ANYA_CLUSTER_MONITORING_LOGGING_ENDPOINT={config['clustering']['monitoring']['logging']['endpoint']}
ANYA_CLUSTER_MONITORING_LOGGING_LEVEL={config['clustering']['monitoring']['logging']['level']}

# Network Configuration
ANYA_CLUSTER_NETWORK_ENCRYPTION_ENABLED={str(config['clustering']['network']['encryption']['enabled']).lower()}
ANYA_CLUSTER_NETWORK_ENCRYPTION_TYPE={config['clustering']['network']['encryption']['type']}
ANYA_CLUSTER_NETWORK_ENCRYPTION_CERT_PATH={config['clustering']['network']['encryption']['cert_path']}
ANYA_CLUSTER_NETWORK_ENCRYPTION_KEY_PATH={config['clustering']['network']['encryption']['key_path']}
"""
        with open(self.env_file, 'w') as f:
            f.write(env_content)

    def view_current_config(self):
        """View current configuration (without sensitive data)."""
        if not self.config_file.exists():
            print("No configuration found. Run setup first.")
            return

        with open(self.config_file, 'r') as f:
            config = yaml.safe_load(f)
            print("\nCurrent Configuration:")
            print(yaml.dump(config, default_flow_style=False))

def main():
    parser = argparse.ArgumentParser(description='Anya Core Sensitive Data Management')
    parser.add_argument('action', choices=['setup', 'update', 'view'],
                      help='Action to perform: setup (new installation), update (existing config), or view')
    
    args = parser.parse_args()
    manager = AnyaSensitiveDataManager()
    
    if args.action == 'setup':
        manager.setup_new_installation()
    elif args.action == 'update':
        manager.update_existing_config()
    elif args.action == 'view':
        manager.view_current_config()

if __name__ == "__main__":
    main()
