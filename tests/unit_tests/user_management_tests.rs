/// The above code contains asynchronous Rust test functions for user management and wallet integration
/// with admin access control.
/// 
/// Returns:
/// 
/// The code provided contains asynchronous test functions for various user management scenarios. Each
/// test function returns a `Result<()>`, indicating that the test runs successfully if no errors occur.
use std::collections::HashMap;

use anyhow::Result;

/// Loads the test configuration asynchronously.
/// 
/// # Returns
/// 
/// * `Result<Config>` - The test configuration wrapped in a Result.
/// 
/// # Errors
/// 
/// This function will return an error if the configuration cannot be loaded.
async fn load_test_config() -> Result<Config> {
    Config::load_test_config().await
}

/// Tests the creation of a new user and verifies that the user can be retrieved.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_creation() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const TEST_USERNAME: &str = "test_user";
    const TEST_PASSWORD: &str = "password123";
    let user = user_manager.create_user(TEST_USERNAME, TEST_PASSWORD, UserRole::Standard).await?;
    assert_eq!(user.username, "test_user");
    assert!(user_manager.get_user(TEST_USERNAME).await.is_ok());
    
    Ok(())
}
/// Tests the user authentication process, ensuring that valid credentials are accepted
/// and invalid credentials are rejected.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_authentication() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const AUTH_TEST_USER: &str = "auth_test_user";
    const SECURE_PASSWORD: &str = "secure_password";
    
    let authenticated = user_manager.authenticate(AUTH_TEST_USER, SECURE_PASSWORD).await?;
    assert!(authenticated);
    
    let wrong_password = user_manager.authenticate(AUTH_TEST_USER, "wrong_password").await?;
    assert!(!wrong_password);
    
    Ok(())
}

/// Tests the user role management process, ensuring that user roles can be updated correctly.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_role_management() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const ROLE_TEST_USERNAME: &str = "role_test_user";
    
    let user = user_manager.create_user(ROLE_TEST_USERNAME, "password123", UserRole::Standard).await?;
    assert_eq!(user.role, UserRole::Standard);
    
    user_manager.update_user_role(ROLE_TEST_USERNAME, UserRole::Admin).await?;
    let updated_user = user_manager.get_user(ROLE_TEST_USERNAME).await?;
    assert_eq!(updated_user.role, UserRole::Admin);
    
    Ok(())
}
/// Tests the deletion of a user and verifies that the user can no longer be retrieved.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_deletion() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const DELETE_TEST_USERNAME: &str = "delete_test_user";
    const DELETE_TEST_PASSWORD: &str = "password123";
    user_manager.create_user(DELETE_TEST_USERNAME, DELETE_TEST_PASSWORD, UserRole::Standard).await?;
    
    user_manager.delete_user(DELETE_TEST_USERNAME).await?;
    assert!(user_manager.get_user(DELETE_TEST_USERNAME).await.is_err());
    
    Ok(())
}
/// Tests the integration of user management with wallet functionality, ensuring that wallet addresses can be retrieved.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_wallet_integration() -> Result<()> { 2)]
async fn test_user_wallet_integration() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const WALLET_TEST_USERNAME: &str = "wallet_test_user";
    const WALLET_TEST_PASSWORD: &str = "password123";
    let user = user_manager.create_user(WALLET_TEST_USERNAME, WALLET_TEST_PASSWORD, UserRole::Standard).await?;
    
    // Assuming wallet is part of user_manager or user
    let wallet = user_manager.get_wallet(&user).await?;
    
    assert!(
        wallet.get_bitcoin_address().is_ok()
        && wallet.get_stacks_address().is_ok()
        && wallet.get_lightning_node_id().is_ok()
    );
    
    Ok(())
}
/// Tests the admin access control, ensuring that only users with the Admin role can access the admin panel.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_admin_access() -> Result<()> {r_threads = 2)]
async fn test_admin_access() -> Result<()> {
    let config = load_test_config().await?;
    let user_manager = UserManager::new(&config)?;
    
    const STANDARD_USER_USERNAME: &str = "standard_user";
    const STANDARD_USER_PASSWORD: &str = "password123";
    const ADMIN_USER_USERNAME: &str = "admin_user";
    const ADMIN_USER_PASSWORD: &str = "admin_pass";
    
    let standard_user = user_manager.create_user(STANDARD_USER_USERNAME, STANDARD_USER_PASSWORD, UserRole::Standard).await?;
    let admin_user = user_manager.create_user(ADMIN_USER_USERNAME, ADMIN_USER_PASSWORD, UserRole::Admin).await?;
    
    // Check if the admin user has access to the admin panel.
    // This method should return true for users with the Admin role.
    assert!(admin_user.can_access_admin_panel().await);
    
    Ok(())
}