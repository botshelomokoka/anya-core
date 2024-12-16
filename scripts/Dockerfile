# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Install Python and pip
RUN apt-get update && apt-get install -y python3 python3-pip

# Install Python dependencies
RUN python3 -m pip install --upgrade pip
RUN pip3 install -r requirements.txt

# Build the Rust project
RUN cargo build --release

# Run the Rust tests
RUN cargo test

# Command to run the executable
CMD ["cargo", "run", "--release"]
