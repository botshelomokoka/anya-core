use std::process::Command;
use std::str;
use tempfile::NamedTempFile;
// The IPFS struct is a marker for IPFS-related operations.
pub struct IPFS;te;

pub struct IPFS;

impl IPFS {
    pub fn new() -> Result<Self, ()> {
        Ok(Self)
    }
    const TEMP_FILE_PATH: &str = "/tmp/ipfs_temp_file";

    pub fn store(&self, data: &[u8]) -> Result<String, ()> {
        // Create a unique temporary file
        let mut temp_file = NamedTempFile::new().map_err(|_| ())?;
        temp_file.write_all(data).map_err(|_| ())?;
        let temp_file_path = TEMP_FILE_PATH;
        let temp_file_path = temp_file.path().to_str().ok_or(())?;

        // Add the file to IPFS
        let output = Command::new("ipfs")
            .arg("add")
            .arg("-q")
            .arg(temp_file_path)
            .output()
            .map_err(|_| ())?;

        // Parse the output to get the IPFS hash
        let hash = str::from_utf8(&output.stdout).map_err(|_| ())?.trim().to_string();

        Ok(hash)
    }
}