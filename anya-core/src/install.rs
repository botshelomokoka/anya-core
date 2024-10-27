use anyhow::Result;
use std::env;
use std::path::PathBuf;

pub struct InstallConfig {
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub secure_storage_type: SecureStorageType,
}

#[derive(Debug, Clone, Copy)]
pub enum SecureStorageType {
    Native,
    Fallback,
}

pub fn setup() -> Result<InstallConfig> {
    let os = env::consts::OS;
    println!("Detected OS: {}", os);

    // Set up directories based on OS
    let (data_dir, config_dir) = match os {
        "linux" => (
            dirs::data_dir().unwrap_or_default().join("anya"),
            dirs::config_dir().unwrap_or_default().join("anya")
        ),
        "windows" => (
            dirs::data_local_dir().unwrap_or_default().join("Anya"),
            dirs::config_dir().unwrap_or_default().join("Anya")
        ),
        "macos" => (
            dirs::data_dir().unwrap_or_default().join("anya"),
            dirs::config_dir().unwrap_or_default().join("anya")
        ),
        _ => (
            dirs::data_local_dir().unwrap_or_default().join("anya"),
            dirs::config_dir().unwrap_or_default().join("anya")
        ),
    };

    // Create directories
    std::fs::create_dir_all(&data_dir)?;
    std::fs::create_dir_all(&config_dir)?;

    // Determine secure storage type
    let secure_storage_type = match os {
        "linux" | "windows" | "macos" => SecureStorageType::Native,
        _ => SecureStorageType::Fallback,
    };

    // Set up OS-specific configurations
    #[cfg(target_os = "linux")]
    {
        // Set up Linux-specific items (keyring, etc.)
        setup_linux()?;
    }

    #[cfg(target_os = "windows")]
    {
        // Set up Windows-specific items
        setup_windows()?;
    }

    #[cfg(target_os = "macos")]
    {
        // Set up macOS-specific items
        setup_macos()?;
    }

    Ok(InstallConfig {
        data_dir,
        config_dir,
        secure_storage_type,
    })
}

#[cfg(target_os = "linux")]
fn setup_linux() -> Result<()> {
    // Check for and set up Linux keyring
    use secret_service::SecretService;
    let ss = SecretService::new(secret_service::EncryptionType::Dh)?;
    ss.get_default_collection()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn setup_windows() -> Result<()> {
    // Check Windows Credential Manager access
    use winapi::um::wincred::CredEnumerateW;
    unsafe {
        let mut count = 0;
        let mut creds = std::ptr::null_mut();
        CredEnumerateW(std::ptr::null(), 0, &mut count, &mut creds);
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn setup_macos() -> Result<()> {
    // Check Keychain access
    use security_framework::os::macos::keychain::SecKeychain;
    SecKeychain::default()?;
    Ok(())
}
