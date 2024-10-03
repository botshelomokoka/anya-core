use anya_core::user_management::{UserManager, User, UserRole};
use anya_core::config::Config;
use anyhow::Result;
use std::collections::HashMap;

#[tokio::test]
async fn test_user_creation() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    let user = user_manager.create_user("test_user", "password123", UserRole::Standard).await?;
    assert_eq!(user.username, "test_user");
    assert_eq!(user.role, UserRole::Standard);
    Ok(())
}

#[tokio::test]
async fn test_user_authentication() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    user_manager.create_user("auth_test_user", "secure_password", UserRole::Standard).await?;
    
    let authenticated = user_manager.authenticate("auth_test_user", "secure_password").await?;
    assert!(authenticated);
    
    let wrong_password = user_manager.authenticate("auth_test_user", "wrong_password").await?;
    assert!(!wrong_password);
    
    Ok(())
}

#[tokio::test]
async fn test_user_role_management() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    let user = user_manager.create_user("role_test_user", "password123", UserRole::Standard).await?;
    assert_eq!(user.role, UserRole::Standard);
    
    user_manager.update_user_role("role_test_user", UserRole::Admin).await?;
    let updated_user = user_manager.get_user("role_test_user").await?;
    assert_eq!(updated_user.role, UserRole::Admin);
    
    Ok(())
}

#[tokio::test]
async fn test_user_deletion() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    user_manager.create_user("delete_test_user", "password123", UserRole::Standard).await?;
    assert!(user_manager.get_user("delete_test_user").await.is_ok());
    
    user_manager.delete_user("delete_test_user").await?;
    assert!(user_manager.get_user("delete_test_user").await.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_user_wallet_integration() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    let user = user_manager.create_user("wallet_test_user", "password123", UserRole::Standard).await?;
    let wallet = user.get_wallet();
    
    assert!(wallet.get_bitcoin_address().is_ok());
    assert!(wallet.get_stacks_address().is_ok());
    assert!(wallet.get_lightning_node_id().is_ok());
    
    Ok(())
}

#[tokio::test]
async fn test_user_permissions() -> Result<()> {
    let config = Config::load_test_config()?;
    let user_manager = UserManager::new(&config)?;
    
    let standard_user = user_manager.create_user("standard_user", "password123", UserRole::Standard).await?;
    let admin_user = user_manager.create_user("admin_user", "admin_pass", UserRole::Admin).await?;
    
    assert!(!standard_user.can_access_admin_panel());
    assert!(admin_user.can_access_admin_panel());
    
    Ok(())
}
