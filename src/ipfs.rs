use std::process::Command;
use std::str;

pub struct IPFS;

impl IPFS {
    pub fn new() -> Result<Self, ()> {
        Ok(Self)
    }

    pub fn store(&self, data: &[u8]) -> Result<String, ()> {
        // Write data to a temporary file
        let temp_file_path = "/tmp/ipfs_temp_file";
        std::fs::write(temp_file_path, data).map_err(|_| ())?;

        // Add the file to IPFS
        let output = Command::new("ipfs")
            .arg("add")
            .arg("-q")
            .arg(temp_file_path)
            .output()
            .map_err(|_| ())?;

        // Parse the output to get the IPFS hash
        let hash = str::from_utf8(&output.stdout).map_err(|_| ())?.trim().to_string();

        // Clean up the temporary file
        std::fs::remove_file(temp_file_path).map_err(|_| ())?;

        Ok(hash)
    }
}