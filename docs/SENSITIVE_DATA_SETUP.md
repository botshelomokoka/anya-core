# Anya Core Sensitive Data Management

This guide explains how to set up and manage sensitive data for your Anya Core installation.

## Prerequisites

- Python 3.7 or higher
- pip (Python package installer)

## Installation

1. Install required packages:
```bash
pip install -r scripts/requirements.txt
```

## Usage

The sensitive data management script provides three main commands:

### 1. Initial Setup

For new installations:
```bash
python scripts/setup_sensitive_data.py setup
```

This will:
- Create configuration directories
- Generate secure secrets
- Set up database credentials
- Configure API keys
- Set up email settings
- Create environment variables

### 2. Update Configuration

To update existing configuration:
```bash
python scripts/setup_sensitive_data.py update
```

This allows you to:
- Update database credentials
- Rotate API keys
- Change email settings
- Generate new secrets

### 3. View Configuration

To view current (non-sensitive) configuration:
```bash
python scripts/setup_sensitive_data.py view
```

## Configuration Storage

The script stores configuration in several locations:

- `~/.anya/sensitive_config.yml`: Non-sensitive configuration
- `~/.anya/secrets.yml`: Sensitive configuration (encrypted)
- `.env`: Environment variables for your application

## Security Best Practices

1. Never commit sensitive files:
   - `.env`
   - `secrets.yml`
   - Any files containing API keys or passwords

2. Use environment variables in production
3. Regularly rotate secrets and API keys
4. Keep your configuration files secure
5. Use strong passwords

## Troubleshooting

If you encounter issues:

1. Check file permissions
2. Verify Python version
3. Ensure all required packages are installed
4. Check configuration file locations
