# Common Issues

## Installation Issues

### Rust Installation Issues

#### Error: Permission denied for rustup-init.sh

If you are getting a permission denied error when attempting to run `rustup-init.sh`, you may need to run `chmod +x ./rustup-init.sh` to make the file executable.

#### Error: Could not download rustup-init.sh

If you are getting an error that `rustup-init.sh` could not be downloaded, you may need to check your internet connection and try again.

### Database Installation Issues

#### Error: Could not connect to Postgres

If you are getting an error that you could not connect to Postgres, you may need to check that Postgres is installed and running on your machine. If you are using a Postgres container in Docker, you may need to make sure that the container is running.

## Running Issues

### Error: Could not find bitcoin-cli

If you are getting an error that `bitcoin-cli` could not be found, you may need to install a Bitcoin Core client and make sure that `bitcoin-cli` is in your `PATH`.

### Error: Could not connect to Bitcoin Core

If you are getting an error that you could not connect to Bitcoin Core, you may need to check that Bitcoin Core is installed and running on your machine. If you are using a Bitcoin Core container in Docker, you may need to make sure that the container is running and that the `bitcoin-cli` command is in your `PATH`.

## General Troubleshooting

### Check the logs

If you are experiencing an issue that is not listed here, you can try checking the logs for more information. The logs are located in the `logs` directory.
